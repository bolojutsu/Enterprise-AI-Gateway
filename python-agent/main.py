import grpc
import gateway_pb2
import gateway_pb2_grpc
import time


class GatewayAgent:
    def __init__(self, host="localhost", port=50051):
        # Establish the gRPC channel
        self.channel = grpc.insecure_channel(f"{host}:{port}")
        # Note: Ensure the stub name matches your proto-generated class
        self.stub = gateway_pb2_grpc.LLMServiceStub(self.channel)

    def execute_prompt(self, prompt, mode="race"):
        """Sends a request to the Rust Gateway and returns the text response."""
        # The Rust service now branches based on the 'model' field
        request = gateway_pb2.PromptRequest(user_prompt=prompt, model=mode)
        try:
            # Match the method name 'execute_prompt' from your Rust service
            response = self.stub.ExecutePrompt(request)
            return response.text
        except grpc.RpcError as e:
            return f"❌ gRPC Error: {e.details()}"

    def run_smart_query(self, query):
        """Agent logic: Auto-detects if research is needed."""
        print(f"\n🧠 Agent received query: '{query}'")

        # Heuristic: If the query asks for real-time/latest data, use research mode
        research_triggers = ["latest", "current", "news", "stock", "price"]
        if any(word in query.lower() for word in research_triggers):
            print("🔍 Strategy: Research Mode (Tavily + Gemini)")
            mode = "research"
        else:
            print("⚡ Strategy: Model Race (Fastest response)")
            mode = "race"

        start_time = time.time()
        result = self.execute_prompt(query, mode=mode)
        duration = time.time() - start_time

        print(f"✅ Completed in {duration:.2f}s")
        return result


if __name__ == "__main__":
    agent = GatewayAgent()

    # Test Query 1: General Knowledge (Triggers Race)
    print(agent.run_smart_query("Explain recursion in one sentence."))

    # Test Query 2: Real-time Data (Triggers Research)
    print(agent.run_smart_query("What is the current price of Bitcoin?"))
