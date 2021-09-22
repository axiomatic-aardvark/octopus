Feature: Server Time

  Scenario: Fetch from API
    Given I send a request to fetch the server time
    When The server time is returned
    Then It is equal to the current UTC time