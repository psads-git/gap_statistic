use kmeans;
use gap_statistic;

#[test]
fn test_optimalk() {
    let data = vec![
       vec![-1.34274827,  4.00374599, -3.01927188],
       vec![-3.83210102,  2.79528826, -3.26948365],
       vec![-2.14055673, -6.69454657,  3.67183855],
       vec![-2.29053006, -7.57211545, -3.47482104],
       vec![-3.617637  , -7.14466599, -5.49755559],
       vec![-3.1599592 , -5.2519727 ,  1.47045989],
       vec![-3.47461404,  2.48267112, -5.01286793],
       vec![-1.29498564, -6.9138905 ,  2.38743915],
       vec![-2.26821565, -9.09811509,  3.42880523],
       vec![-3.1181572 , -8.44780257, -4.61462131],
       vec![-2.8514013 , -7.42865659, -3.81255726],
       vec![-2.33663987, -8.45630196, -4.16432476],
       vec![-3.49354572, -6.719973  ,  3.76594187],
       vec![-3.04759022, -8.31142229,  0.37461329],
       vec![-4.18257105, -6.99000865,  1.60222837],
       vec![-4.76908098, -7.10992238, -5.82643227],
       vec![-2.79277598,  2.69183652, -3.13861159],
       vec![-3.01162586,  2.19776552, -4.75497051],
       vec![-3.98787309,  4.15729736, -4.31176666],
       vec![-4.30293715, -9.25934248, -5.11361269]
    ];
;

    let result = gap_statistic::optimal_k(data, (1..10).collect());
    println!("Got {:?}", result)
}