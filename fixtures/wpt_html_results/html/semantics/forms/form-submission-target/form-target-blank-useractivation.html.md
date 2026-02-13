# html/semantics/forms/form-submission-target/form-target-blank-useractivation.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-target/form-target-blank-useractivation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Test that submitting a target=_blank form consumes userActivation</title>
<link rel="help" href="https://html.spec.whatwg.org/#concept-form-submit">
<link rel="help" href="https://html.spec.whatwg.org/#the-rules-for-choosing-a-navigable">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<div id=log></div>
<form action=resources/endpoint.html target=_blank>
  <input type=hidden name=channelname>
  <button>Submit</button>
</form>
<script>

function waitForNewWindow(aChannelName) {
  return new Promise(resolve => {
    let channel = new BroadcastChannel(aChannelName);
    channel.addEventListener("message", () => {
      assert_true(true, "new window is opened");
      channel.postMessage("close");
      resolve();
    }, {once: true});
  });
}

function testFormSubmission(aChannelName, aSubmitFun, aMsg) {
  promise_test(async () => {
    document.querySelector("input").value = aChannelName;

    await test_driver.bless('transient activation');
    assert_true(navigator.userActivation.isActive, 'should have user activation');

    let newWindowPromise = waitForNewWindow(aChannelName);
    aSubmitFun();
    await newWindowPromise;
    assert_false(navigator.userActivation.isActive, 'navigator.userActivation.isActive after opening a new window');
  }, aMsg);
}

testFormSubmission(`${Date.now()}_script_submit`, () => {
  document.querySelector("form").submit();
}, `<form target=_blank>.submit()`);

  testFormSubmission(`${Date.now()}_script_requestSubmit`, () => {
  document.querySelector("form").requestSubmit();
}, `<form target=_blank>.requestSubmit()`);

  testFormSubmission(`${Date.now()}_click_submit`, () => {
  test_driver.click(document.querySelector("button"));
}, `<form target=_blank> click submit`);

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
  "source_name": "html/semantics/forms/form-submission-target/form-target-blank-useractivation.html"
}
```
