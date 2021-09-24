Feature: Server Time

  Scenario: Fetch from API (Success)
    Given I send a request to fetch the server time
    When The server time is returned
    Then It is equal to the current UTC time

  Scenario: Fetch from API (Fail)
    Given I send a request to fetch the server time with a bad url
    When A bad endpoint error is returned
    Then The bad endpoint error is reported properly