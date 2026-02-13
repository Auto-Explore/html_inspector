# html/capability-delegation/delegation-consumes-activation.https.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/capability-delegation/delegation-consumes-activation.https.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<!--
   Tentative due to:
     https://github.com/whatwg/html/issues/4008
-->
<title>Capability Delegation: Consumes User Activation</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/utils.js"></script>

<div>
  Test that capability delegation consumes transient user activation.

  https://wicg.github.io/capability-delegation/spec.html
</div>

<iframe width="300px" height="50px"></iframe>

<script>
  function sendCapabilityDelegationMessageIgnoringException(origin, capability) {
      try {
          frames[0].postMessage("any_message", {targetOrigin: origin, delegate: capability});
      } catch (e) {}
  }

  let capability_to_delegate;

  promise_setup(async () => {
      capability_to_delegate = await findOneCapabilitySupportingDelegation();
      assert_true(!!capability_to_delegate, "The user agent supports delegating at least one capability");
  });

  promise_test(async () => {
      assert_false(navigator.userActivation.isActive);

      await test_driver.bless();
      assert_true(navigator.userActivation.isActive, "User activation is available initially");

      sendCapabilityDelegationMessageIgnoringException("/", "blah");
      assert_true(navigator.userActivation.isActive,
                  "User activation is not consumed by delegation of an unknown delegation");

      sendCapabilityDelegationMessageIgnoringException("*", capability_to_delegate);
      assert_true(navigator.userActivation.isActive,
                  "User activation is not consumed by known delegation to disallowed targetOrigin");

      sendCapabilityDelegationMessageIgnoringException("/", capability_to_delegate);
      assert_false(navigator.userActivation.isActive,
                   "User activation is consumed by supported delegation");

  }, "Capability delegation consumes transient user activation");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/capability-delegation/delegation-consumes-activation.https.tentative.html"
}
```
