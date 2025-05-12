# 🪑 Seat Yatra – A Concurrent Ticket Booking Challenge (Rust Assignment)

Welcome to **Seat Yatra**, a Rust-based ticket booking simulation designed to test your understanding of concurrency, state safety, and realistic system modeling.

## 🎭 Context

**Seat Yatra** is an Indian cultural theatre event with limited seats and thousands of eager attendees. Your job is to build a **highly concurrent ticket booking system** that:

- Manages thousands of ticket requests.
- Ensures that **no seat is double-booked**.
- **Simulates real-world behavior** like payment failures and session retries.
- Guarantees that **all tickets are sold** by the end. Given that we have atleast one Request for that ticket.

---

## 🧩 Problem Statement

Design and implement a ticket booking system that:

### Generates:

- ✅ A fixed number of `Ticket`s, each with a unique seat number.
- ✅ A list of `User`s that simulate booking sessions.
- ✅ A stream of `RequestTickets`, where each user requests 1–3 seats.

### Handles:

- 🔄 **Concurrent booking attempts** (simulate with threads).
- ⚠️ **Payment failures** (simulated with 70% success probability).
- 🔁 **Session retries** for users whose booking fails (due to conflicts or payment).
- 📜 **Logs** of booking attempts per user, including retries.

### Ensures:

- ✅ **No seat is double-assigned**.
- ✅ **All tickets are eventually sold**.
- ✅ Successful bookings generate a `Receipt`.

---
