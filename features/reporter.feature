Feature: Report

  Scenario: Generate a report (success)
    Given All three components are fetched successfully
    When I generate a report without errors
    Then The report without errors is correctly generated

  Scenario: Generate a report (1 component fails)
    Given One component fails to be fetched
    When I generate a report with one error and two successful reports
    Then The report with with one error and two successful reports is correctly generated

  Scenario: Generate a report (all components fail)
    Given All components fail to be fetched
    When I generate a report with three error messages
    Then The report with three error messages is correctly generated