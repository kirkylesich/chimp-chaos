# Implementation Plan

- [x] 1. Set up project structure and core modules
  - Create MVC directory structure (models, views, controllers, scenarios)
  - Set up main.rs with basic tokio runtime and tracing initialization
  - Create module declarations and basic exports
  - _Requirements: 2.1, 8.3_

- [x] 2. Implement core type system with phantom types
  - [x] 2.1 Create strongly typed identifiers in models/identifiers.rs
    - Implement ExperimentId, PodId, NodeId newtype wrappers
    - Add serde serialization support for all ID types
    - _Requirements: 3.5_

  - [x] 2.2 Implement phantom type state machine in models/state_machine.rs
    - Create phantom type markers (Pending, Running, Completed, Failed)
    - Implement Experiment<State> wrapper with type-safe transitions
    - Add From implementations for valid state transitions
    - _Requirements: 3.1, 3.2, 3.4_

  - [x] 2.3 Define core error types with anyhow integration
    - Create ChaosError enum with comprehensive error variants
    - Implement error conversion traits for external crate errors
    - Add error context and chain support
    - _Requirements: 10.1, 10.2, 10.3, 10.4_

- [x] 3. Create ChaosExperiment CRD definition
  - [x] 3.1 Implement ChaosExperiment CRD in models/chaos_experiment.rs
    - Define ChaosExperimentSpec struct with kube CustomResource derive
    - Add schemars JsonSchema support for OpenAPI generation
    - Implement serde serialization with proper field validation
    - _Requirements: 1.1, 1.2, 2.2, 8.3_

  - [x] 3.2 Create scenario type definitions in models/scenario_types.rs
    - Define ScenarioType enum with all supported chaos scenarios
    - Implement TargetSpec for workload selection
    - Add parameter validation structures
    - _Requirements: 4.2, 4.3, 4.4, 4.5_

- [x] 4. Implement ChaosScenario trait system
  - [x] 4.1 Define ChaosScenario trait in scenarios/traits.rs
    - Create async trait with validate, execute, and cleanup methods
    - Define ExecutionResult and ScenarioMetadata types
    - Add trait bounds for Send + Sync requirements
    - _Requirements: 8.1, 8.2_

  - [x] 4.2 Create scenario factory in scenarios/factory.rs
    - Implement ScenarioFactory with HashMap-based registration
    - Add dynamic scenario creation based on ScenarioType
    - Implement inventory-based automatic registration system
    - _Requirements: 8.2, 8.5_

  - [x] 4.3 Implement basic print-based scenarios
    - Create PodKillerScenario that prints execution details
    - Create CpuStressScenario that simulates stress testing
    - Create NetworkDelayScenario that logs network operations
    - _Requirements: 7.2, 7.3, 7.4_

- [ ] 5. Build Kubernetes controller infrastructure
  - [ ] 5.1 Create experiment controller in controllers/experiment_controller.rs
    - Implement ExperimentController trait with reconcile method
    - Add experiment lifecycle management with type-safe state transitions
    - Integrate with scenario factory for experiment execution
    - _Requirements: 1.4, 2.3, 7.1_

  - [ ] 5.2 Implement Kubernetes reconciliation loop in controllers/reconciler.rs
    - Set up kube Controller with ChaosExperiment resource watching
    - Implement reconciliation logic with error handling and retries
    - Add proper cleanup and finalizer handling
    - _Requirements: 1.1, 1.3, 5.4_

  - [ ] 5.3 Create state manager in controllers/state_manager.rs
    - Implement StateManager trait for experiment state persistence
    - Add Kubernetes status updates with proper error handling
    - Integrate with phantom type system for compile-time safety
    - _Requirements: 1.4, 3.2, 3.4_

- [ ] 6. Implement HTTP API views
  - [ ] 6.1 Create API handlers in views/api_handlers.rs
    - Set up actix-web server with experiment CRUD endpoints
    - Implement JSON serialization for all API responses
    - Add proper error handling and HTTP status codes
    - _Requirements: 2.4, 5.1, 5.2_

  - [ ] 6.2 Add event publishing in views/event_publisher.rs
    - Implement Kubernetes event creation for experiment state changes
    - Add structured event data with proper metadata
    - Integrate with experiment controller for automatic event publishing
    - _Requirements: 9.2_

- [ ] 7. Set up agent communication framework
  - [ ] 7.1 Create agent client in views/agent_client.rs
    - Implement HTTP client using reqwest with JSON serialization
    - Add retry logic with exponential backoff for failed requests
    - Create strongly typed request/response structures
    - _Requirements: 5.1, 5.2, 5.3, 5.4_

  - [ ] 7.2 Implement agent health monitoring
    - Add periodic health check functionality
    - Create agent status tracking and reporting
    - Integrate health status with experiment execution decisions
    - _Requirements: 5.5_

- [ ] 8. Add comprehensive logging and observability
  - [ ] 8.1 Set up structured logging with tracing
    - Configure tracing-subscriber with JSON formatting
    - Add tracing spans for all major operations
    - Implement log level configuration from environment
    - _Requirements: 9.1, 9.4_

  - [ ] 8.2 Create metrics exposition
    - Add basic metrics for experiment execution
    - Implement experiment duration and success rate tracking
    - Create metrics endpoint for Prometheus scraping
    - _Requirements: 9.3_

- [ ] 9. Wire everything together and create main application
  - [ ] 9.1 Implement main.rs with complete application setup
    - Initialize tokio runtime with proper signal handling
    - Set up tracing and logging configuration
    - Start Kubernetes controller and HTTP API server concurrently
    - _Requirements: 7.1, 7.5_

  - [ ] 9.2 Add configuration management
    - Create OperatorConfig struct with environment variable support
    - Implement configuration validation and defaults
    - Add runtime configuration loading and error handling
    - _Requirements: 9.1_

  - [ ] 9.3 Create application integration tests
    - Set up integration test framework with test Kubernetes cluster
    - Test complete experiment lifecycle from creation to completion
    - Validate API endpoints and controller reconciliation
    - _Requirements: 7.5_

- [ ]* 9.4 Add comprehensive unit tests
  - Write unit tests for all scenario implementations
  - Test phantom type state machine transitions
  - Create mock implementations for external dependencies
  - _Requirements: 3.4, 7.5_