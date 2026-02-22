# 🚀 Hybrid Enterprise AI Gateway & Model Arena

A high-performance, polyglot AI orchestration system. This project demonstrates a production-grade architecture that bridges the gap between **high-speed systems infrastructure (Rust)** and **flexible AI logic (Python)** using **gRPC**, **REST**, and **asynchronous persistence**.



## 🌟 The Core Problem Solved
Traditional AI prototypes built entirely in Python suffer from the **Global Interpreter Lock (GIL)** and high memory overhead. This project implements a **Hybrid Architecture**:
* **Rust (The Muscle):** Acts as the "High-Speed Engine." It handles concurrent networking, security, and parallel task execution via `tokio`.
* **Python (The Brain):** Acts as the "Orchestrator." It manages complex reasoning, prompt engineering, and agent logic.

---

## 🌟 Key Technical Features

### 🏎️ Competitive Concurrency (The "Model Race")
Unlike traditional failover systems that wait for a timeout, this gateway implements a **Race Pattern**. Using Rust's `tokio::select!`, the gateway fires requests to multiple LLM providers (OpenAI, Gemini, Claude, Grok) simultaneously.
* **Zero-Latency Switching:** Returns the result from whichever model finishes first.
* **Automatic Cancellation:** The "losing" futures are instantly dropped to save resources.

### 🔍 Grounded Research Mode
The gateway includes an intelligent "Research" pathway. When triggered, it utilizes the **Tavily API** for real-time web search and pipes the context into **Gemini 1.5 Flash** for a grounded, hallucination-free summary.

### 📊 Real-Time Analytics & Persistence
* **SQLite + SQLx:** Every request, winner, and latency metric is logged into a local SQLite database using non-blocking asynchronous queries.
* **Live Dashboard:** A Vite/React frontend visualizes provider performance via **Recharts**, showing a live leaderboard of which AI is currently "winning" the race.

### 🌉 Polyglot Bridge (gRPC + Protobuf)
* **Protobuf:** Acts as the strictly-typed "Contract" between Rust and Python, ensuring binary-speed data transfer.
* **Shared State:** Manages global metrics across gRPC and REST thread pools using **Atomic Reference Counting (`Arc`)**.

---

## 🏗️ System Architecture



1.  **Frontend (Vite + React):** Real-time dashboard monitoring system health, request logs, and model status.
2.  **Gateway (Rust + Axum + Tonic):** A multi-threaded multiplexer running gRPC (internal) and REST (external) simultaneously.
3.  **Agent Logic (Python):** A dedicated service for AI reasoning and prompt construction, communicating via high-speed binary buffers.

---

## 🛠️ Tech Stack

| Component | Technology | Role |
| :--- | :--- | :--- |
| **Infrastructure** | **Rust / Tokio** | Concurrency, Security, & Proxying |
| **Logic** | **Python** | AI Agent Reasoning & Prompt Engineering |
| **Frontend** | **React / JSX** | Real-time Monitoring & Win-rate Charts |
| **Database** | **SQLite / SQLx** | Asynchronous Performance Logging |
| **Transport** | **gRPC / Protobuf** | High-speed Internal Binary Bridge |

---

## 📂 Project Structure

```text
ai-enterprise-project/
├── proto/              # Shared gRPC definitions (.proto)
├── rust-gateway/       # Performance Layer (Axum & Tonic)
│   ├── migrations/     # SQLx database schema migrations
│   └── src/main.rs     # Multi-threaded Multiplexer (REST + gRPC)
├── python-agent/       # Reasoning Layer (Agent Logic)
│   └── main.py         # gRPC Client & AI Logic
└── vite-project/       # Visualization Layer (React)
    └── src/App.jsx     # Dashboard UI with Live Feeds