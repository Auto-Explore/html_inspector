# html/semantics/embedded-content/the-iframe-element/iframe_sandbox_allow_script.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_allow_script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: iframe_sandbox_allow_scripts</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#attr-iframe-sandbox">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-iframe-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>
<script>
  // Set up all our script stuff before the iframe starts loading, so we don't
  // miss any messages from it.
  var step1 = false;
  var t = async_test("iframe_sandbox_allow_scripts");

  setup({timeout:1000});
  window.addEventListener("message", callback1, false);

  function run() {
    window.removeEventListener("message", callback1, false);
    document.getElementById("testIframe").sandbox = "allow-scripts";
    document.getElementById("testIframe").contentWindow.location.reload();
    window.addEventListener("message", callback2, false);
  }

  function callback1(e) {
    step1= !step1;
  }

  function callback2(e) {
    t.step(function () {
      assert_false(step1, "[allow-scripts] is not set.");
      assert_equals(e.data, "Script executed", "[allow-scripts] is set.");
    });
    t.done();
  }

  // Make sure the iframe loads before we mess with it.
  window.addEventListener("load", function() {
    // The load event might fire before a message from the child comes in...
    // Wait a bit to see if that message does come in.
    setTimeout(run, 500);
  });
</script>
<iframe id="testIframe" src="support/sandbox_allow_script.html" sandbox="allow-same-origin" style="display:none"></iframe>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_sandbox_allow_script.html"
}
```
