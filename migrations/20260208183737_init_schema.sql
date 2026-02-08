CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  email TEXT UNIQUE NOT NULL,
  password_hash TEXT NOT NULL,
  role TEXT NOT NULL CHECK (role IN ('customer', 'owner')),
  phone TEXT,
  created_at TIMESTAMP DEFAULT now()
);

CREATE TABLE hotels (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  owner_id UUID NOT NULL REFERENCES users(id),
  name TEXT NOT NULL,
  description TEXT,
  city TEXT NOT NULL,
  country TEXT NOT NULL,
  amenities TEXT[] DEFAULT ARRAY[]::TEXT[],
  rating NUMERIC(2,1) DEFAULT 0.0,
  total_reviews INT DEFAULT 0,
  created_at TIMESTAMP DEFAULT now()
);

CREATE TABLE rooms (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  hotel_id UUID NOT NULL REFERENCES hotels(id) ON DELETE CASCADE,
  room_number TEXT NOT NULL,
  room_type TEXT NOT NULL,
  price_per_night NUMERIC(10,2) NOT NULL,
  max_occupancy INT NOT NULL,
  created_at TIMESTAMP DEFAULT now(),
  UNIQUE (hotel_id, room_number)
);

CREATE TABLE bookings (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id),
  room_id UUID NOT NULL REFERENCES rooms(id),
  hotel_id UUID NOT NULL REFERENCES hotels(id),
  check_in_date DATE NOT NULL,
  check_out_date DATE NOT NULL,
  guests INT NOT NULL,
  total_price NUMERIC(10,2) NOT NULL,
  status TEXT DEFAULT 'confirmed'
    CHECK (status IN ('confirmed', 'cancelled')),
  booking_date TIMESTAMP DEFAULT now(),
  cancelled_at TIMESTAMP,
  CHECK (check_out_date > check_in_date)
);

CREATE TABLE reviews (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  booking_id UUID UNIQUE NOT NULL REFERENCES bookings(id),
  user_id UUID NOT NULL REFERENCES users(id),
  hotel_id UUID NOT NULL REFERENCES hotels(id),
  rating INT NOT NULL CHECK (rating BETWEEN 1 AND 5),
  comment TEXT,
  created_at TIMESTAMP DEFAULT now()
);