use tengwar::*;


fn quenya(line: impl AsRef<str>) {
    println!("{}\n", line.to_tengwar::<Quenya>());
}


#[test]
fn test_test() {
    quenya(
        "tinco parma calma qessë ando umbar anga ungwë\
        \nþúlë formen harma hwesta anto ampa anca unqë\
        \nnúmen malta ngoldo ngwalmë orë vala anna wilya\
        \nrómen arda lambë alda silmë ázë essë\
        \nhyarmen yanta úrë ossë halla telco ára"
    );
    quenya("hrívë, hlócë");
}
