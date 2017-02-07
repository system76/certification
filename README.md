# System76 Certification

A collection of automated and manual processes to ensure the delivery of a quality Ubuntu machine.

<div style="page-break-after: always;"></div>

## Rationale

We would like to, for production and engineering reasons, have a set of automated and manual procedures that can be collected with as little user input as possible to identify product issues.

For support reasons, we would like to have a set of tests that support staff can use to identify potential issues with hardware in the field.

For marketing reasons, we would like to have more information to offer potential customers about the effectiveness of our machines.

<div style="page-break-after: always;"></div>

## High Level Concepts

### Product Conception

When we are attempting to produce a new product, we need to evaluate what hardware is present and can be activated, using automated procedures. This information should be collected in a centralized repository.

### Product Development

To develop the product, we then need to run both automated and manual tests, iterating to produce a releasable product. This information should be tracked over time, which we already do with a manual process.

### Product Release

Upon product release, we should be able to publish the results of our test in a beautiful, powerful manner to compliment our other product materials. We should attempt to track user engagement with this information.

### Product Maintenance

When we handle support issues, we should refer to collected test information, and be able to reproduce test results on machines in the field, or in the RMA process.

<div style="page-break-after: always;"></div>

## Low Level Implementation

### Automated Hardware Discovery

We should have a process of, given a new barebones machine, identifying present hardware and determining if there are test cases covering the usage of this hardware

### Standardized Tests

Each class of hardware should have a set of automated and manual tests and benchmarks that can be run in a scripted form. Preference would be to segregate manual and automated tests in order to allow quick runs of either.

### Test Suggestion

For discovered hardware that does not have tests or benchmarks, suggestions should be made and included in the results

### Automated Result Collection

Result collection should be automated, regardless of the automation of the test

### Pretty and "Ugly" interfaces to Results

We should have a manner of presenting test results to users in a highly aesthetic manner, in addition to providing more information for our purposes and those of power users.

### Comparison of Results Across Hardware

We should be able to compare results among different pieces of hardware, allowing us to display benchmark results to customers interested in examining two pieces of hardware.

### Tests Prior to Shipping

We should have sanity testing to identify lemons before shipping them.

### User-Generated Results

We should allow users to run this test tool and produce their own test results.

<div style="page-break-after: always;"></div>

## Possible Development Process

- Create a result collection client for some simple, automatable tests and benchmarks.
- Create a server to host results, per product line (Oryx Pro), and model (8156)
- Create an ugly user interface for test results
- Create a method of discovering present hardware, and adding to the result recommendations for benchmarks and tests.
- Improve upon the number of tests to include the most useful ones
- Create a pretty user interface for test results
- Create a comparison method for test results
- Publicize this new certification framework