# html/capability-delegation/delegation-sender-checks.tentative.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/capability-delegation/delegation-sender-checks.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
!DOCTYPE html>
<!--
   Tentative due to:
     https://github.com/WICG/capability-delegation
-->
<title>Capability Delegation sender checks</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/utils.js"></script>

<div>
  Verifies that capability delegation related error checks in <a
  href="https://wicg.github.io/capability-delegation/spec.html#monkey-patch-to-html-initiating-delegation">HTML
  postMessage algorithm</a> are enforced correctly.
</div>

<iframe width="300px" height="50px"></iframe>

<script>
  const frame = frames[0];
  const message = "any_message";
  const activate = false;

  let capability_to_delegate;

  promise_setup(async () => {
      capability_to_delegate = await findOneCapabilitySupportingDelegation();
      assert_true(!!capability_to_delegate, "The user agent supports delegating at least one capability");
  });

  promise_test(async () => {
      try {
          await postCapabilityDelegationMessage(frame, message, "/", "blah", activate);
          assert_unreached();
      } catch (exception) {
          assert_equals(exception.name, "NotSupportedError");
      }
  }, "Delegating an unsupported capability throws an exception");

  promise_test(async () => {
      try {
          await postCapabilityDelegationMessage(frame, message, "*", capability_to_delegate, activate);
          assert_unreached();
      } catch (exception) {
          assert_equals(exception.name, "NotAllowedError");
      }
  }, "Delegating to targetOrigin='*' throws an exception");

  promise_test(async () => {
      try {
          await postCapabilityDelegationMessage(frame, message, "/", capability_to_delegate, activate);
          assert_unreached();
      } catch (exception) {
          assert_equals(exception.name, "NotAllowedError");
      }
  }, "Delegating without user activation throws an exception");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 103,
        "byte_start": 96,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/capability-delegation/delegation-sender-checks.tentative.html"
}
```
