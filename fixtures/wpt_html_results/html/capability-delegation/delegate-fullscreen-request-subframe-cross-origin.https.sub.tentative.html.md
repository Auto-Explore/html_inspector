# html/capability-delegation/delegate-fullscreen-request-subframe-cross-origin.https.sub.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/capability-delegation/delegate-fullscreen-request-subframe-cross-origin.https.sub.tentative.html",
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
<title>Capability Delegation of Fullscreen Requests: Subframe Cross-Origin</title>
<script src="/common/get-host-info.sub.js"></script>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/utils.js"></script>

<div>
  Verifies that element.requestFullscreen() calls from a cross-origin subframe without user
  activation work if and only if the top frame has user activation and it delegates the capability.

  https://wicg.github.io/capability-delegation/spec.html

  See wpt/html/user-activation/propagation*.html for frame tree user activation visibility tests.
</div>

<iframe allow="fullscreen" width="300px" height="50px">
</iframe>

<script>
  const origin = get_host_info().HTTPS_REMOTE_ORIGIN;
  document.querySelector("iframe").src = origin + "/html/capability-delegation/resources/delegate-fullscreen-request-recipient.html";

  function testCrossOriginSubframeFullscreenDelegation(capability, activate, expectation) {
      const message = {"type": "make-fullscreen-request"};
      const expectationType = expectation ? "succeeds" : "fails";
      const delegationType = capability ? "with delegation" : "without delegation";
      const activationType = activate ? "with user activation" : "with no user activation";

      promise_test(async () => {
          const data = await postCapabilityDelegationMessage(frames[0], message, origin, capability, activate);
          assert_equals(data.result, expectation ? "success" : "failure");
      }, `Fullscreen requests from a cross-origin subframe ${expectationType} ${delegationType} from an opener ${activationType}`);
  }

  promise_setup(async () => {
      // Make sure the recipient iframe has loaded.
      return getMessageData("recipient-loaded", frames[0]);
  });

  testCrossOriginSubframeFullscreenDelegation(/*capability=*/"", /*activate=*/false, /*expectation=*/false);
  testCrossOriginSubframeFullscreenDelegation(/*capability=*/"", /*activate=*/true, /*expectation=*/false);
  testCrossOriginSubframeFullscreenDelegation(/*capability=*/"fullscreen", /*activate=*/true, /*expectation=*/true);
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
  "source_name": "html/capability-delegation/delegate-fullscreen-request-subframe-cross-origin.https.sub.tentative.html"
}
```
