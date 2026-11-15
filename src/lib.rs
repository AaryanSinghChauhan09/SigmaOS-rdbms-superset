/// Apache Superset dashboard and chart engine for SigmaOS.

#[derive(Debug, Clone, PartialEq)]
pub enum ChartType {
    Bar,
    Line,
    Pie,
    Scatter,
    Table,
    Heatmap,
}

#[derive(Debug, Clone)]
pub struct Dataset {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<f64>>,
}

impl Dataset {
    pub fn new(name: &str, columns: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            columns,
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<f64>) -> Result<(), String> {
        if row.len() != self.columns.len() {
            return Err(format!(
                "Row has {} values but dataset has {} columns",
                row.len(),
                self.columns.len()
            ));
        }
        self.rows.push(row);
        Ok(())
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }
}

#[derive(Debug, Clone)]
pub struct Chart {
    pub title: String,
    pub chart_type: ChartType,
    pub dataset_name: String,
    pub x_column: String,
    pub y_columns: Vec<String>,
}

pub struct SupersetDashboard {
    pub name: String,
    datasets: Vec<Dataset>,
    charts: Vec<Chart>,
}

impl SupersetDashboard {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            datasets: Vec::new(),
            charts: Vec::new(),
        }
    }

    pub fn add_dataset(&mut self, dataset: Dataset) {
        self.datasets.push(dataset);
    }

    pub fn add_chart(&mut self, chart: Chart) {
        self.charts.push(chart);
    }

    pub fn chart_count(&self) -> usize {
        self.charts.len()
    }

    pub fn get_dataset(&self, name: &str) -> Option<&Dataset> {
        self.datasets.iter().find(|d| d.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_dataset_aaa() {
        // Arrange
        let mut dashboard = SupersetDashboard::new("Sales Dashboard");
        let dataset = Dataset::new(
            "monthly_sales",
            vec!["month".to_string(), "revenue".to_string()],
        );

        // Act
        dashboard.add_dataset(dataset);

        // Assert
        assert!(dashboard.get_dataset("monthly_sales").is_some());
        assert_eq!(
            dashboard
                .get_dataset("monthly_sales")
                .unwrap()
                .column_count(),
            2
        );
    }

    #[test]
    fn test_add_chart_aaa() {
        // Arrange
        let mut dashboard = SupersetDashboard::new("Analytics");
        let chart = Chart {
            title: "Revenue Over Time".to_string(),
            chart_type: ChartType::Line,
            dataset_name: "monthly_sales".to_string(),
            x_column: "month".to_string(),
            y_columns: vec!["revenue".to_string()],
        };

        // Act
        dashboard.add_chart(chart);

        // Assert
        assert_eq!(dashboard.chart_count(), 1);
    }

    #[test]
    fn test_dataset_row_validation_aaa() {
        // Arrange
        let mut dataset = Dataset::new("metrics", vec!["a".to_string(), "b".to_string()]);

        // Act
        let ok = dataset.add_row(vec![1.0, 2.0]);
        let err = dataset.add_row(vec![1.0, 2.0, 3.0]);

        // Assert
        assert!(ok.is_ok());
        assert!(err.is_err());
        assert_eq!(dataset.row_count(), 1);
    }
}
