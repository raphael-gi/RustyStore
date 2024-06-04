CREATE TABLE customer (
  id SERIAL PRIMARY KEY,
  username VARCHAR(45),
  email VARCHAR(90),
  password VARCHAR(90)
);

CREATE TABLE product (
  id SERIAL PRIMARY KEY,
  name VARCHAR(90),
  price DECIMAL NOT NULL,
  description TEXT
);

INSERT INTO customer (username, email, password)
VALUES ('Chris Bumstead', 'chris@bummy.dev', 'hehe');

INSERT INTO customer (username, email, password)
VALUES ('Kenji', 'kenj@bummy.dev', 'hehe');

CREATE OR REPLACE PROCEDURE register(us VARCHAR, em VARCHAR, password VARCHAR) 
AS $$
DECLARE 
    username_used VARCHAR = NULL;
    email_used VARCHAR = NULL;
BEGIN
  SELECT username, email INTO username_used, email_used FROM customer c WHERE c.username = us OR c.email = em;
  
  IF email_used = em THEN
    RAISE EXCEPTION 'Email "%" is already taken', em USING HINT = 'You might already have an account';
  END IF;
  
  IF username_used = us THEN
    RAISE EXCEPTION 'Username "%" is already taken', us USING HINT = 'Provide another username';
  END IF;
  
  INSERT INTO customer (username, email, password) VALUES (us, em, password);
END;
$$ LANGUAGE plpgsql;


INSERT INTO product (name, price, description) VALUES
('Wireless Mouse', 29.99, 'A wireless mouse with ergonomic design and long battery life.'),
('Mechanical Keyboard', 99.95, 'A mechanical keyboard with customizable RGB backlighting.'),
('USB-C Hub', 49.99, 'A USB-C hub with multiple ports including HDMI, USB 3.0, and Ethernet.'),
('Bluetooth Headphones', 79.99, 'Noise-cancelling over-ear Bluetooth headphones with high-fidelity sound.'),
('4K Monitor', 299.99, 'A 27-inch 4K monitor with ultra-thin bezels and HDR support.'),
('External SSD', 149.99, 'A portable external SSD with 1TB of storage and USB 3.1 interface.'),
('Gaming Chair', 199.99, 'An ergonomic gaming chair with adjustable armrests and lumbar support.'),
('Smartphone Stand', 19.99, 'A smartphone stand with adjustable angles and non-slip base.'),
('Portable Charger', 39.99, 'A portable charger with 10000mAh capacity and fast charging support.'),
('Smartwatch', 249.99, 'A smartwatch with heart rate monitoring and GPS tracking.'),
('Desk Lamp', 34.99, 'A desk lamp with adjustable brightness and color temperature settings.'),
('Wireless Charger', 24.99, 'A wireless charger with fast charging support for Qi-enabled devices.'),
('Webcam', 59.99, 'A 1080p webcam with built-in microphone and low-light correction.'),
('Noise-Cancelling Earbuds', 129.99, 'In-ear noise-cancelling earbuds with long battery life and touch controls.'),
('Fitness Tracker', 99.99, 'A fitness tracker with heart rate monitoring and sleep tracking features.'),
('Electric Scooter', 499.99, 'A foldable electric scooter with a top speed of 15 mph and a range of 20 miles.'),
('Smart Home Speaker', 89.99, 'A smart home speaker with voice assistant integration and high-quality audio.'),
('Action Camera', 199.99, 'A waterproof action camera with 4K video recording and image stabilization.'),
('Laptop Stand', 29.99, 'An adjustable laptop stand with a foldable design and cooling fans.'),
('Wireless Earbuds', 79.99, 'Wireless earbuds with Bluetooth 5.0 and a compact charging case.'),
('Portable Projector', 299.99, 'A portable projector with 1080p resolution and built-in speakers.'),
('Digital Photo Frame', 119.99, 'A digital photo frame with Wi-Fi connectivity and a 10-inch display.'),
('Smart Thermostat', 149.99, 'A smart thermostat with remote control and energy-saving features.'),
('Robot Vacuum', 249.99, 'A robot vacuum with smart mapping and self-charging capabilities.'),
('Smart Light Bulbs', 49.99, 'A set of smart light bulbs with app control and color changing options.'),
('Electric Kettle', 39.99, 'An electric kettle with temperature control and rapid boiling.'),
('Standing Desk', 399.99, 'An adjustable standing desk with memory settings and a sturdy frame.'),
('Wireless Gaming Mouse', 59.99, 'A wireless gaming mouse with customizable buttons and RGB lighting.'),
('Noise-Cancelling Headset', 179.99, 'An over-ear noise-cancelling headset with a detachable microphone.'),
('Streaming Device', 49.99, 'A streaming device with 4K support and access to multiple streaming services.'),
('E-Reader', 129.99, 'An e-reader with a glare-free screen and adjustable front light.'),
('Smart Doorbell', 99.99, 'A smart doorbell with video recording and two-way audio communication.'),
('Portable Speaker', 59.99, 'A waterproof portable speaker with Bluetooth connectivity and deep bass.'),
('Fitness Band', 79.99, 'A fitness band with activity tracking and smartphone notifications.'),
('Gaming Console', 499.99, 'A next-generation gaming console with ultra-high-speed SSD and ray tracing.'),
('Smart Plug', 24.99, 'A smart plug with voice control and energy monitoring features.'),
('Air Purifier', 149.99, 'An air purifier with HEPA filter and smart air quality sensor.'),
('Wireless Router', 129.99, 'A high-speed wireless router with dual-band support and parental controls.'),
('Electric Toothbrush', 89.99, 'An electric toothbrush with multiple cleaning modes and a smart timer.'),
('Bluetooth Tracker', 29.99, 'A Bluetooth tracker to locate your items with a smartphone app.'),
('VR Headset', 299.99, 'A virtual reality headset with high-resolution display and immersive experiences.'),
('3D Printer', 399.99, 'A 3D printer with a large build volume and high-precision printing.'),
('Smart Security Camera', 79.99, 'A smart security camera with night vision and motion detection.'),
('Electric Bike', 899.99, 'An electric bike with pedal assist and a long-lasting battery.'),
('Indoor Garden', 149.99, 'An indoor garden system with automatic watering and grow lights.'),
('Smart Mirror', 299.99, 'A smart mirror with touchscreen display and voice assistant integration.'),
('Electric Blanket', 79.99, 'An electric blanket with adjustable heat settings and auto shut-off.'),
('Portable Air Conditioner', 249.99, 'A portable air conditioner with remote control and dehumidifier function.'),
('Solar Charger', 49.99, 'A solar charger with USB ports and foldable design for outdoor use.'),
('Electric Grill', 119.99, 'An electric grill with non-stick surface and temperature control.'),
('Smart Scale', 59.99, 'A smart scale with body composition analysis and app connectivity.'),
('Wireless Printer', 149.99, 'A wireless printer with high-speed printing and cloud connectivity.'),
('Smart Coffee Maker', 129.99, 'A smart coffee maker with programmable settings and Wi-Fi control.'),
('Noise Machine', 39.99, 'A noise machine with multiple sound options for better sleep.'),
('Digital Piano', 499.99, 'A digital piano with weighted keys and multiple sound settings.'),
('Smart Lock', 149.99, 'A smart lock with keyless entry and smartphone control.'),
('Cordless Vacuum', 199.99, 'A cordless vacuum with powerful suction and long battery life.'),
('Electric Shaver', 79.99, 'An electric shaver with precision blades and waterproof design.'),
('Smart Water Bottle', 49.99, 'A smart water bottle with hydration tracking and LED reminders.'),
('Portable Generator', 299.99, 'A portable generator with multiple outlets and USB ports.'),
('Smart Pet Feeder', 99.99, 'A smart pet feeder with scheduled feeding and portion control.'),
('Photo Printer', 129.99, 'A photo printer with wireless connectivity and high-quality prints.'),
('Air Fryer', 99.99, 'An air fryer with adjustable temperature and digital controls.'),
('Electric Pressure Cooker', 89.99, 'An electric pressure cooker with multiple cooking functions.'),
('Smart LED Strip', 39.99, 'A smart LED strip with app control and color changing options.'),
('Portable Power Bank', 29.99, 'A portable power bank with high capacity and fast charging.'),
('Smart Remote', 69.99, 'A smart remote control with voice commands and universal compatibility.'),
('Indoor Security Camera', 49.99, 'An indoor security camera with 1080p resolution and motion alerts.'),
('Electric Skillet', 59.99, 'An electric skillet with non-stick surface and temperature control.'),
('Smart Air Purifier', 199.99, 'A smart air purifier with real-time air quality monitoring.'),
('Wireless Doorbell', 29.99, 'A wireless doorbell with long range and multiple chime options.'),
('Smart Garden', 99.99, 'A smart garden kit with self-watering and grow lights.'),
('Electric Wine Opener', 29.99, 'An electric wine opener with rechargeable battery and foil cutter.'),
('Wireless Security System', 199.99, 'A wireless security system with cameras and door/window sensors.'),
('Smart Garage Door Opener', 79.99, 'A smart garage door opener with remote control and app notifications.');
