pub fn ascii(id: i32, daytime: &str) -> Vec<String> {
    let sunny: Vec<String> = vec![
        "  \\ | /   ".to_string(),
        "  - O -   ".to_string(),
        "  / | \\   ".to_string(),
    ];

    let partial_clouds_day: Vec<String> = vec![
        "  \\ /(  ) ".to_string(),
        " - O(    ) ".to_string(),
        "  / (  )  ".to_string(),
    ];
    let partial_clouds_night: Vec<String> = vec![
        "  *   (  ) ".to_string(),
        ".   (    ) ".to_string(),
        "   * (  )  ".to_string(),
    ];

    let clouds: Vec<String> = vec![
        "  ( )()_  ".to_string(),
        " (      ) ".to_string(),
        "  ( )()   ".to_string(),
    ];

    let night: Vec<String> = vec![
        "    .*   * ".to_string(),
        "  * .      ".to_string(),
        "  . . * .  ".to_string(),
    ];

    let drizzle: Vec<String> = vec![
        " '   '     '".to_string(),
        "  '    ' ' ".to_string(),
        "'      '   '".to_string(),
    ];

    let rain: Vec<String> = vec![
        " ' '' ' '  ".to_string(),
        " '' ' ' '  ".to_string(),
        " ' ' '' '  ".to_string(),
    ];

    let thunderstorm: Vec<String> = vec![
        " ''_/ _/'  ".to_string(),
        " ' / _/' ' ".to_string(),
        " /_/'' ''  ".to_string(),
    ];

    let chaos: Vec<String> = vec![
        " c__ ''' ' ".to_string(),
        " ' '' c___ ".to_string(),
        " c__ ' 'c_ ".to_string(),
    ];

    let snow: Vec<String> = vec![
        " * '* ' * ".to_string(),
        " '* ' * '  ".to_string(),
        " *' * ' * ".to_string(),
    ];

    let fog: Vec<String> = vec![
        " -- _ --   ".to_string(),
        " -__-- -   ".to_string(),
        " - _--__   ".to_string(),
    ];

    match id {
        199..233 => thunderstorm,
        299..321 => drizzle,
        499..532 => rain,
        599..623 => snow,
        700..781 => fog,
        801 => match daytime {
            "day" => partial_clouds_day,
            _ => partial_clouds_night,
        },
        802 => match daytime {
            "day" => partial_clouds_day,
            _ => partial_clouds_night,
        },
        803 => match daytime {
            "day" => partial_clouds_day,
            _ => partial_clouds_night,
        },
        804 => clouds,
        _ => match daytime {
            "day" => sunny,
            "night" => night,
            _ => chaos,
        },
    }
}
