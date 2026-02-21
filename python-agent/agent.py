import grpc
import gateway_pb2
import gateway_pb2_grpc
import time


class GatewayAgent:
    def __init__(self):
        # Connect to the Rust server on the same port we set in main.rs
        self.channel = grpc.insecure_channel("localhost:50051")
        self.stub = gateway_pb2_grpc.LLMServiceStub(self.channel)

    def run(self, user_input: str):
        # 🚥 SIMPLE ROUTING LOGIC
        # If the user asks for news or facts, we could force a 'search' model
        # If it's a general question, we let Rust run the 'race'

        print(f"🤖 Agent thinking about: {user_input}")

        # In a real agent, you'd use a small model (like Haiku) to decide the intent.
        # For now, let's use a keyword-based intent router:
        if "search" in user_input.lower() or "latest" in user_input.lower():
            model_target = "tavily"
        else:
            model_target = "race-mode"

        # Prepare the gRPC request
        request = gateway_pb2.PromptRequest(user_prompt=user_input, model=model_target)

        try:
            # 🚀 Call the Rust Gateway!
            response = self.stub.ExecutePrompt(request)
            return response.text
        except grpc.RpcError as e:
            return f"❌ Gateway Error: {e.details()}"


if __name__ == "__main__":
    agent = GatewayAgent()
    query = "Who won the game last night?"
    print(f"Final Answer: {agent.run(query)}")
