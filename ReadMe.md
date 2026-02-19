# 🚀 Hybrid Enterprise AI Gateway

A high-performance, polyglot AI orchestration system. This project demonstrates a production-grade architecture that bridges the gap between **high-speed systems infrastructure (Rust)** and **flexible AI logic (Python)** using **gRPC** and **REST**.



## 🌟 The Core Problem Solved
Traditional AI prototypes built entirely in Python suffer from the **Global Interpreter Lock (GIL)** and high memory overhead, leading to performance bottlenecks when handling parallel LLM calls or high user traffic. 

This project implements a **Hybrid Architecture** to solve this:
* **Rust (The Muscle):** Acts as the "Outer Shield" and "High-Speed Engine." It handles concurrent networking, security, and parallel task execution via `tokio`.
* **Python (The Brain):** Acts as the "Orchestrator." It manages complex reasoning, prompt engineering, and AI library integrations (like LangChain or CrewAI).

---

## 🏗️ System Architecture

The system utilizes a **three-tier microservice approach**:

1.  **Frontend (Vite + React/JSX):** A real-time monitoring dashboard that displays system health, request logs, and model status.
2.  **Gateway (Rust + Axum + Tonic):** * **Multiplexer:** Simultaneously runs a gRPC server (internal) and a REST API (external).
    * **Parallelism:** Uses `tokio::spawn` to "scatter" requests across multiple LLM providers concurrently, significantly reducing total latency.
3.  **Agent Logic (Python + gRPC):** A dedicated service for AI reasoning, communicating with the gateway via high-speed binary **Protocol Buffers**.



---

## 🛠️ Tech Stack

| Component      | Technology         | Role                                      |
| :------------- | :----------------- | :---------------------------------------- |
| **Infrastructure** | **Rust** | Concurrency, Security, & Proxying         |
| **Logic** | **Python** | AI Agent Reasoning & Tooling              |
| **Frontend** | **React (JSX)** | Real-time Monitoring & UI                 |
| **Transport** | **gRPC / Protobuf**| High-speed Internal Binary Bridge         |
| **Runtime** | **Tokio** | Asynchronous Work-Stealing Scheduler      |

---

## 📂 Project Structure

```text
ai-enterprise-project/
├── proto/              # Shared gRPC Service Definitions (.proto)
├── rust-gateway/       # Performance Layer (Axum & Tonic)
│   ├── src/main.rs     # Multi-threaded Multiplexer (REST + gRPC)
│   └── build.rs        # Automated Bridge Generation
├── python-agent/       # Reasoning Layer (Agent Logic)
│   └── main.py         # gRPC Client & AI Logic
└── ui/                 # Visualization Layer (Vite/JSX)
    └── src/App.jsx     # Dashboard UI