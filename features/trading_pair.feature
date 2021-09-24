Feature: Trading Pair

  Scenario: Fetch from API
    Given I send a request to fetch the trading pair info for XBT-USD
    When The trading pair info is returned
    Then The trading pair response is valid

  Scenario: Fetch from API (Fail)
    Given I send a request to fetch trading pair info with a non-existent pair
    When A non-existent pair error is returned
    Then The non-existent pair error is reported properly