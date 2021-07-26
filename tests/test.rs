use tengwar::*;


fn quenya(line: impl AsRef<str>) {
    println!("{}", line.to_tengwar::<Quenya>());
}


fn gondor(line: impl AsRef<str>) {
    println!("{}", line.to_tengwar::<Gondor>());
}


#[test]
fn test_quenya() {
    quenya("quenya:");
    quenya(
        "tinco parma calma qessë ando umbar anga ungwë\
        \nþúlë formen harma hwesta anto ampa anca unqë\
        \nnúmen malta ngoldo ngwalmë orë vala anna wilya\
        \nrómen arda lambë alda silmë ázë essë\
        \nhyarmen yanta úrë ossë halla telco ára"
    );
    // quenya("hrívë, hlócë");
}


#[test]
fn test_gondor() {
    gondor("edhellen:");
    gondor(
        "tau pui cí dau bui gí thau afui\
        \nacho adho ampa anca nau mui engi orë\
        \nvala anna awae-feleg aro rhau alo lhau sau\
        \nsau esso hau iau úrë ossë\
        \nhalla chae dwae gwae"
    );
    // gondor("hrívë, hlócë");
}
