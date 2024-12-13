use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Apple iPhone 16 Pro ".to_string(),
            price: 1699.99,
            description: "Strength. Beauty. Titanium. Hello, Apple Intelligence. A18 Pro chip. Camera Control. Buy now. Take a closer look. Get to know iPhone 16 Pro. 48MP Ultra Wide camera. Titanium design. iOS 18. Siri. A18 Pro chip.".to_string(),
            image: "/catnip.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Apple Watch Series S10".to_string(),
            price: 599.99,
            description: "Series 10 is a major milestone for Apple Watch. It features our biggest and most advanced display yet, 1 showing more information onscreen than ever.".to_string(),
            image: "/squid.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Apple MacBook M2 Chip".to_string(),
            price: 1299.99,
            description: "The M2 is made with TSMC's "Enhanced 5-nanometer technology" N5P process and contains 20 billion transistors, a 25% increase from the M1. Apple claims CPU improvements up to 18% and GPU improvements up to 35% compared to the M1.".to_string(),
            image: "/mermaid.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Apple Airpods Pro 3rd Gen".to_string(),
            price: 249.99,
            description: "Apple Airpods Pro 3rd Gen".to_string(),
            image: "/ocean.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Apple Mac mini 3rd Gen".to_string(),
            price: 899.99,
            description: "Looks small. Lives large. Front and back ports. Shop Mac mini with M4 and M4 Pro chips.".to_string(),
            image: "/pirate.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Apple HomePods 2nd Gen".to_string(),
            price: 699.99,
            description: "The Apple HomePod is a wireless speaker and Siri virtual assistant that can be used to play music, control smart home devices, and more".to_string(),
            image: "/tug.jpg".to_string()
        },
        Product {
            id: 7,
            name: "SONY PS5 Play Station Pro".to_string(),
            price: 899.99,
            description: "The Sony PlayStation 5 Pro is a gaming console with improved graphics, performance, and features compared to the standard PlayStation 5".to_string(),
            image: "/bed.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Bolt Pro".to_string(),
            price: 7.99,
            description: "An electric bike, or ebike, is a bicycle equipped with an electric bike motor to assist you when you're pedalling".to_string(),
            image: "/knot.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Boat Headphones".to_string(),
            price: 299.99,
            description: "Headphones are a pair of small loudspeaker drivers worn on or around the head over a user's ears".to_string(),
            image: "/crabby.jpg".to_string()
        },
        Product {
            id: 10,
            name: "Apple Airtag ".to_string(),
            price: 29.99,
            description: "Apple airtags".to_string(),
            image: "/lifejacket.jpg".to_string()
        }
    ]
}
