# html/capability-delegation/delegate-fullscreen-request-popup-same-origin.https.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/capability-delegation/delegate-fullscreen-request-popup-same-origin.https.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<!--
   Tentative due to:
     https://github.com/WICG/capability-delegation/issues/10
-->
<title>Capability Delegation of Fullscreen Requests: Popup Same-Origin</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/utils.js"></script>

<div>
  Verifies that element.requestFullscreen() calls from a same-origin popup without user activation
  work if and only if the opener has user activation and it delegates the capability.

  https://wicg.github.io/capability-delegation/spec.html
</div>

<script>
  let popup = null;

  function testSameOriginPopupFullscreenDelegation(capability, activate, expectation) {
      const message = {"type": "make-fullscreen-request"};
      const expectationType = expectation ? "succeeds" : "fails";
      const delegationType = capability ? "with delegation" : "without delegation";
      const activationType = activate ? "with user activation" : "with no user activation";

      promise_test(async () => {
          const data = await postCapabilityDelegationMessage(popup, message, location.origin, capability, activate);
          assert_equals(data.result, expectation ? "success" : "failure");
      }, `Fullscreen requests from a same-origin popup ${expectationType} ${delegationType} from an opener ${activationType}`);
  }

  promise_setup(async () => {
      // Make sure the recipient popup has loaded.
      popup = window.open("./resources/delegate-fullscreen-request-recipient.html",
                          "", "width=300,height=200");
      return getMessageData("recipient-loaded", popup);
  });

  testSameOriginPopupFullscreenDelegation(/*capability=*/"", /*activate=*/false, /*expectation=*/false);
  testSameOriginPopupFullscreenDelegation(/*capability=*/"", /*activate=*/true, /*expectation=*/false);
  testSameOriginPopupFullscreenDelegation(/*capability=*/"fullscreen", /*activate=*/true, /*expectation=*/true);
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
  "source_name": "html/capability-delegation/delegate-fullscreen-request-popup-same-origin.https.tentative.html"
}
```
