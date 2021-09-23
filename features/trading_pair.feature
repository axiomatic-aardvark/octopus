Feature: Trading Pair

  Scenario: Fetch from API
    Given I send a request to fetch the trading pair info for XBT-USD
    When The trading pair info is returned
    Then The trading pair response is valid