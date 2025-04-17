use cubing::{alg::parse_alg, puzzles::cube2x2x2_kpuzzle};

pub fn main() {
    use twsearch::{
        _internal::cli::args::GodsAlgorithmOptionalArgs, // TODO
        experimental_lib_api::gods_algorithm,
    };

    let kpuzzle = cube2x2x2_kpuzzle();
    let mut optional_args = GodsAlgorithmOptionalArgs::default();
    optional_args.generator_args.generator_moves_string = Some("U,F2,R".to_owned());
    let table = gods_algorithm(kpuzzle, optional_args).unwrap();

    // Looking up any pattern is now O(1).
    let depth = table.pattern_to_depth.get(
        &kpuzzle
            .default_pattern()
            .apply_alg(parse_alg!("U' F R' F' U R' U' R F2 U'"))
            .unwrap(),
    );
    dbg!(depth);
}
