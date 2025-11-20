import { Text, Box } from "ink";
import { useModels } from "../hooks/use-models.js";
import { Spinner, Select, Alert } from "@inkjs/ui";
import React, { useEffect, useState } from "react";
import TextInput from "ink-text-input";
import { useAppState } from "../context/app-context.js";

const steps = [
  "name",
  "model",
  "promptPath",
  "useRag",
  "embeddingModel",
  "ragFilesPath",
  "done",
] as const;

export const CreateAssistant = () => {
  const { models } = useModels();
  const { resetCommandChain } = useAppState();
  const [currentStepIndex, setCurrentStepIndex] = useState(0);

  const [name, setName] = useState("");
  const [model, setModel] = useState("");
  const [promptPath, setPromptPath] = useState("");
  const [useRag, setUseRag] = useState(false);
  const [embeddingModel, setEmbeddingModel] = useState("");
  const [ragFilesPath, setRagFilesPath] = useState("");
  const currentStep = steps[currentStepIndex];

  useEffect(() => {
    if (currentStep === "done") {
      setTimeout(() => {
        resetCommandChain();
      }, 1200);
    }
  }, [currentStep]);

  const nextStep = () => setCurrentStepIndex(currentStepIndex + 1);

  const handleNameSubmit = (value: string) => {
    if (value.trim()) {
      setName(value);
      nextStep();
    }
  };

  const handleModelSelect = (value: string) => {
    setModel(value);
    nextStep();
  };

  const handlePromptPathSubmit = (value: string) => {
    if (value.trim()) {
      setPromptPath(value);
      nextStep();
    }
  };

  const handleUseRagSelect = (value: string) => {
    const approved = value === "Yes";
    setUseRag(approved);
    if (approved) {
      nextStep();
    } else {
      // Skip RAG steps
      setCurrentStepIndex(steps.indexOf("done"));
    }
  };

  const handleEmbeddingModelSelect = (value: string) => {
    setEmbeddingModel(value);
    nextStep();
  };

  const handleRagFilesPathSubmit = (value: string) => {
    if (value.trim()) {
      setRagFilesPath(value);
      nextStep();
    }
  };

  const renderCurrentStep = () => {
    switch (currentStep) {
      case "name":
        return (
          <Box>
            <Text>Enter assistant name: </Text>
            <TextInput
              value={name}
              onChange={setName}
              onSubmit={handleNameSubmit}
            />
          </Box>
        );
      case "model":
        return models ? (
          <>
            <Text>Select a model:</Text>
            <Select
              options={models.map((m) => ({ value: m.name, label: m.name }))}
              onChange={handleModelSelect}
            />
          </>
        ) : (
          <Spinner label="Loading models..." />
        );
      case "promptPath":
        return (
          <Box>
            <Text>Enter path to system prompt file: </Text>
            <TextInput
              value={promptPath}
              onChange={setPromptPath}
              onSubmit={handlePromptPathSubmit}
            />
          </Box>
        );
      case "useRag":
        return (
          <>
            <Text>Add RAG (Retrieval-Augmented Generation)?</Text>
            <Select
              options={[
                { label: "Yes", value: "Yes" },
                { label: "No", value: "No" },
              ]}
              onChange={handleUseRagSelect}
            />
          </>
        );
      case "embeddingModel":
        return models ? (
          <>
            <Text>Select an embedding model for RAG:</Text>
            <Select
              options={models.map((m) => ({ value: m.name, label: m.name }))}
              onChange={handleEmbeddingModelSelect}
            />
          </>
        ) : (
          <Spinner label="Loading models..." />
        );
      case "ragFilesPath":
        return (
          <Box>
            <Text>Enter path to files for RAG: </Text>
            <TextInput
              value={ragFilesPath}
              onChange={setRagFilesPath}
              onSubmit={handleRagFilesPathSubmit}
            />
          </Box>
        );
      case "done":
        return (
          <Alert variant={"success"}>Assistant configured successfully!</Alert>
        );
      default:
        return null;
    }
  };

  return (
    <Box flexDirection="column">
      <Box
        flexDirection="column"
        borderStyle="single"
        paddingX={1}
        marginBottom={1}
      >
        <Text bold>New Assistant Configuration</Text>
        <Text>
          Name: <Text color={name ? "white" : "gray"}>{name || "..."}</Text>
        </Text>
        {currentStepIndex > 0 && (
          <Text>
            Model:{" "}
            <Text color={model ? "white" : "gray"}>{model || "..."}</Text>
          </Text>
        )}
        {currentStepIndex > 1 && (
          <Text>
            System Prompt:{" "}
            <Text color={promptPath ? "white" : "gray"}>
              {promptPath || "..."}
            </Text>
          </Text>
        )}
        {currentStepIndex > 2 && (
          <Text>
            RAG Enabled: <Text color="white">{useRag ? "Yes" : "No"}</Text>
          </Text>
        )}
        {useRag && currentStepIndex > 3 && (
          <Text>
            Embedding Model:{" "}
            <Text color={embeddingModel ? "white" : "gray"}>
              {embeddingModel || "..."}
            </Text>
          </Text>
        )}
        {useRag && currentStepIndex > 4 && (
          <Text>
            RAG Source Path:{" "}
            <Text color={ragFilesPath ? "white" : "gray"}>
              {ragFilesPath || "..."}
            </Text>
          </Text>
        )}
      </Box>

      {renderCurrentStep()}
    </Box>
  );
};
