#!/usr/bin/perl
use Mojo::Base -strict, -signatures;

use Mojo::File qw(path);
use Mojo::Log;
use Mojo::UserAgent;
use Mojo::SQLite;

use Text::CSV qw(csv);

my ($db_file, $orders_file) = @ARGV;
die "Usage: $0 [db_file] [orders_file]\n" unless $db_file && $orders_file;

my $log = Mojo::Log->new;
my $ua = Mojo::UserAgent->new;
my $sqlite = Mojo::SQLite->new($db_file);
my $db = $sqlite->db;

# ordered_at, id, cost, content, img_url, order_url, product_url
my $csv = csv in => $orders_file, headers => "auto";
my %uniq;
for my $order (@$csv) {
  next if $uniq{"$order->{id}:$order->{content}"}++;

  # Download order image
  my $ext = $order->{img_url} =~ /\.(\w+)$/ ? $1 : "jpg";
  my $img = path qw(static images orders), "$order->{id}.$ext";
  if ($order->{img_url} =~ /pixel/) {
    $log->info("Can't download $order->{img_url} to $img");
    unlink $img if -e $img;
  } elsif (!-e $img) {
    $log->info("Downloading $order->{img_url} to $img");
    my $res = $ua->get($order->{img_url})->result;
    $res->content->asset->move_to($img);
  }

  my @tags = (qw(amazon order yen), $order->{id} =~ s/\W//gr);
  push @tags, 'giftcard' if length($order->{cost}) && !$order->{cost};
  $order->{content} .= join ' ', '', map { "#$_" } @tags;
  $order->{cost} ||= 0;
  $order->{img_url} = "/images/orders/$order->{id}.$ext";

  $db->query("insert into moments
    (created_at, ext_id, content, cost, ext_url, img_url) values (?, ?, ?, ?, ?, ?)
    on conflict (ext_id, lower(content)) do update set
    content = ?, cost = ?, ext_url = ?, img_url = ?",
    "$order->{ordered_at} 00:00:00",
    @$order{qw(id content cost product_url img_url content cost product_url img_url)});

  for my $tag (@tags) {
    $db->query("insert into moment_tags
      (moment_id, kind, name)
      values ((select id from moments where ext_id = ? and content = ? limit 1), 'tag', ?)
      on conflict (moment_id, kind, lower(name)) do update set name = ?",
      @$order{qw(id content)}, $tag, $tag);
  }
}
