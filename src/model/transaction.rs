use serde::{Deserialize, Serialize};

use super::Address;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: Address,    // Represents "Batch ID" (e.g., WHEAT-001)
    pub recipient: Address, // Represents "Location/Actor" (e.g., WAREHOUSE-A)
    pub data: String,       // NEW: Represents "Agri Details" (JSON String)
    pub batch_id: String,
    pub event_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::test_util::{alice, bob};

    fn farm_address() -> Address {
        alice()
    }

    fn warehouse_address() -> Address {
        bob()
    }

    #[test]
    fn should_create_transaction() {
        let tx = Transaction {
            sender: farm_address(),
            recipient: warehouse_address(),
            data: r#"{"quantity": "100kg", "quality": "Grade A"}"#.to_string(),
            batch_id: "WHEAT-001".to_string(),
            event_type: "HARVEST".to_string(),
        };

        assert_eq!(tx.sender, farm_address());
        assert_eq!(tx.recipient, warehouse_address());
        assert_eq!(tx.batch_id, "WHEAT-001");
        assert_eq!(tx.event_type, "HARVEST");
    }

    #[test]
    fn should_clone_transaction() {
        let tx1 = Transaction {
            sender: farm_address(),
            recipient: warehouse_address(),
            data: r#"{"temperature": "4C", "humidity": "65%"}"#.to_string(),
            batch_id: "CORN-042".to_string(),
            event_type: "STORAGE".to_string(),
        };

        let tx2 = tx1.clone();

        assert_eq!(tx1.sender, tx2.sender);
        assert_eq!(tx1.recipient, tx2.recipient);
        assert_eq!(tx1.data, tx2.data);
        assert_eq!(tx1.batch_id, tx2.batch_id);
        assert_eq!(tx1.event_type, tx2.event_type);
    }

    #[test]
    fn should_serialize_to_json() {
        let tx = Transaction {
            sender: farm_address(),
            recipient: warehouse_address(),
            data: r#"{"location": "Warehouse-A", "inspector": "John Doe"}"#.to_string(),
            batch_id: "RICE-999".to_string(),
            event_type: "QUALITY_CHECK".to_string(),
        };

        let json = serde_json::to_string(&tx).unwrap();
        assert!(json.contains("RICE-999"));
        assert!(json.contains("QUALITY_CHECK"));
    }

    #[test]
    fn should_deserialize_from_json() {
        let json = r#"{
            "sender": "f780b958227ff0bf5795ede8f9f7eaac67e7e06666b043a400026cbd421ce28e",
            "recipient": "51df097c03c0a6e64e54a6fce90cb6968adebd85955917ed438e3d3c05f2f00f",
            "data": "{\"vehicle\": \"TRUCK-42\", \"distance\": \"50km\"}",
            "batch_id": "WHEAT-123",
            "event_type": "TRANSPORT"
        }"#;

        let tx: Transaction = serde_json::from_str(json).unwrap();
        assert_eq!(tx.batch_id, "WHEAT-123");
        assert_eq!(tx.event_type, "TRANSPORT");
        assert!(tx.data.contains("TRUCK-42"));
    }

    #[test]
    fn should_handle_harvest_event() {
        let tx = Transaction {
            sender: farm_address(),
            recipient: farm_address(),
            data: r#"{"crop": "wheat", "quantity": "500kg", "field": "Field-7"}"#.to_string(),
            batch_id: "WHEAT-2024-001".to_string(),
            event_type: "HARVEST".to_string(),
        };

        assert_eq!(tx.event_type, "HARVEST");
        assert!(tx.data.contains("wheat"));
    }

    #[test]
    fn should_handle_processing_event() {
        let tx = Transaction {
            sender: warehouse_address(),
            recipient: farm_address(),
            data: r#"{"process": "milling", "output": "450kg flour"}"#.to_string(),
            batch_id: "WHEAT-2024-001".to_string(),
            event_type: "PROCESSING".to_string(),
        };

        assert_eq!(tx.event_type, "PROCESSING");
        assert!(tx.data.contains("milling"));
    }

    #[test]
    fn should_handle_transport_event() {
        let tx = Transaction {
            sender: farm_address(),
            recipient: warehouse_address(),
            data: r#"{"driver": "Jane Smith", "vehicle": "TRUCK-15", "departure": "2024-12-22T08:00:00Z"}"#.to_string(),
            batch_id: "CORN-042".to_string(),
            event_type: "TRANSPORT".to_string(),
        };

        assert_eq!(tx.event_type, "TRANSPORT");
        assert!(tx.data.contains("TRUCK-15"));
    }

    #[test]
    fn should_handle_complex_agricultural_data() {
        let complex_data = r#"{
            "temperature": 25,
            "humidity": 60,
            "location": "Warehouse-A",
            "pesticide_free": true,
            "organic": true,
            "certifications": ["USDA", "EU-Organic"]
        }"#;

        let tx = Transaction {
            sender: warehouse_address(),
            recipient: warehouse_address(),
            data: complex_data.to_string(),
            batch_id: "ORGANIC-WHEAT-001".to_string(),
            event_type: "QUALITY_CHECK".to_string(),
        };

        assert_eq!(tx.event_type, "QUALITY_CHECK");

        // Verify the data field contains valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&tx.data).unwrap();
        assert_eq!(parsed["temperature"], 25);
        assert_eq!(parsed["organic"], true);
    }
}

