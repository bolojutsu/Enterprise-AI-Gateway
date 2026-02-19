import grpc
import gateway_pb2
import gateway_pb2_grpc


def run_agent():
    # Connect to the Rust Gateway over the gRPC bridge
    with grpc.insecure_channel("localhost:50051") as channel:
        stub = gateway_pb2_grpc.LLMServiceStub(channel)

        # This is where your Agent Logic would go
        print("Agent is 'thinking'...")

        # Call the Rust Gateway
        request = gateway_pb2.PromptRequest(
            model="gpt-4o", user_prompt="Explain multi-threading to a Java dev."
        )

        response = stub.ExecutePrompt(request)
        print(f"Response from Rust Gateway: {response.text}")
        print(f"Calculated Cost: ${response.cost}")


if __name__ == "__main__":
    run_agent()
