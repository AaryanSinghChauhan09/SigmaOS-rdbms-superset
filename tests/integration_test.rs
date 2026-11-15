use sigmaos_rdbms_superset::{Chart, ChartType, Dataset, SupersetDashboard};

#[test]
fn test_add_dataset_aaa() {
    // Arrange
    let mut dashboard = SupersetDashboard::new("Main Dashboard");
    let mut ds = Dataset::new(
        "orders",
        vec!["date".to_string(), "total".to_string(), "count".to_string()],
    );
    ds.add_row(vec![20240101.0, 1500.0, 42.0]).unwrap();
    ds.add_row(vec![20240102.0, 2000.0, 55.0]).unwrap();

    // Act
    dashboard.add_dataset(ds);

    // Assert
    let found = dashboard.get_dataset("orders").unwrap();
    assert_eq!(found.row_count(), 2);
    assert_eq!(found.column_count(), 3);
}

#[test]
fn test_add_chart_aaa() {
    // Arrange
    let mut dashboard = SupersetDashboard::new("Charts Demo");
    for ct in [ChartType::Bar, ChartType::Pie, ChartType::Scatter] {
        let chart = Chart {
            title: format!("{:?} chart", ct),
            chart_type: ct,
            dataset_name: "ds".to_string(),
            x_column: "x".to_string(),
            y_columns: vec!["y".to_string()],
        };
        dashboard.add_chart(chart);
    }

    // Act + Assert
    assert_eq!(dashboard.chart_count(), 3);
}

#[test]
fn test_dataset_row_validation_aaa() {
    // Arrange
    let mut ds = Dataset::new("test", vec!["col1".to_string()]);

    // Act
    let ok = ds.add_row(vec![42.0]);
    let err = ds.add_row(vec![1.0, 2.0]);

    // Assert
    assert!(ok.is_ok());
    assert!(err.is_err());
}
