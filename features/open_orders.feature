Feature: Open Orders

  Scenario: Fetch from API
    Given I send a request to fetch the open orders for an account
    When The open orders are returned
    Then The open orders response is valid

  Scenario: Fetch from API (Fail)
    Given I send a request to fetch the open orders for an account with invalid otp
    When A invalid signature error is returned
    Then The invalid signature error is reported properly