Feature: Open Orders

  Scenario: Fetch from API
    Given I send a request to fetch the open orders for an account
    When The open orders are returned
    Then The response is valid