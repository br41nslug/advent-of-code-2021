use strict;
use Data::Dumper;

sub calc_fuel {
    my ($distance) = @_;
    my $result = 0;
    for (my $i = 1; $i <= $distance; $i++) {
        $result += $i;
    }
    return $result;
}

sub calc_distances {
    my (%crabs) = @_;
    my %distances;
    my @keys = sort keys %crabs;
    my @range = ($keys[0]..$keys[$#keys]);
    foreach (@range) {
        my $distance = 0;
        my $pos = $_;
        for (sort keys %crabs) {
            my $val = calc_fuel(abs($_ - $pos));
            $distance += $val * $crabs{$_};
        }
        $distances{$pos} = $distance;
    }
    return %distances;
}

my %crabs;
$crabs{$_}++ for split(',', @ARGV[0]);

my %distances = calc_distances(%crabs);
my @sorted = sort { $a <=> $b } values %distances;
my $winner = @sorted[0];
print "Least fuel used: $winner\n";
