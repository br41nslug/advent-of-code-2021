use strict;
use Data::Dumper;

sub calc_distances {
    my (%crabs) = @_;
    my %distances;
    for (sort keys %crabs) {
        my $distance = 0;
        my $pos = $_;
        for (sort keys %crabs) {
            my $val = abs($_ - $pos);
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
