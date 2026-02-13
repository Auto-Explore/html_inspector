# html/semantics/embedded-content/the-iframe-element/iframe_javascript_url_xhr.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe_javascript_url_xhr.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>javascript: URL triggers sync xhr</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#process-the-iframe-attributes">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
let iframeLoaded = false;
</script>
<iframe onload="iframeLoaded = true; this.onload = assert_unreached;"></iframe>
<script>

let javascriptUrlRan = false;
window.javascriptURLXhr = function() {
  // sychronous XHR
  var xhr = new XMLHttpRequest();
  xhr.open('GET', '/xhr/resources/delay.py?ms=2000', false);
  xhr.send();

  javascriptUrlRan = true;
  setTimeout(() => {
    parent.document.dispatchEvent(new Event("scriptExecuted"));
  }, 0);
};

promise_test(async t => {
  assert_true(iframeLoaded, "iframeLoaded", "Initial iframe load occured synchronously");
  javascriptUrlRan = false;

  let scriptExecutePromise = new Promise(resolve => {
    document.addEventListener("scriptExecuted", function() {
      resolve();
    }, { once: true });
  });

  const iframe = document.querySelector('iframe');

  // Load image on iframe.
  let image = iframe.contentDocument.createElement("img");
  image.src = "/images/blue.png";
  iframe.contentDocument.body.appendChild(image);

  // Javascript URL load on iframe.
  iframe.src = "javascript:void(parent.javascriptURLXhr())";
  assert_false(javascriptUrlRan, "javascriptUrlRan");
  await scriptExecutePromise;
  assert_true(javascriptUrlRan, "javascriptUrlRan");

  // Verify only one load event is fired on iframe
  await new Promise(resolve => step_timeout(() => resolve(), 100));
}, "Image load should not trigger load event for non-initial javascript URL load");

promise_test(async t => {
  javascriptUrlRan = false;
  const iframe = document.createElement("iframe");
  iframe.src = "javascript:void(parent.javascriptURLXhr())";

  let loadEventPromise = new Promise(resolve => {
    iframe.onload = function() {
      assert_true(javascriptUrlRan, "script should have ran");
      this.onload = assert_unreached;
      resolve();
    };
  });
  document.body.appendChild(iframe);
  assert_false(javascriptUrlRan, "javascriptUrlRan");

  // Image load on iframe.
  let image = iframe.contentDocument.createElement("img");
  image.src = "/images/blue.png";
  iframe.contentDocument.body.appendChild(image);

  await loadEventPromise;

  // Verify only one load event is fired on iframe
  await new Promise(resolve => step_timeout(() => resolve(), 100));
}, "Load event for initial javascript URL load should be fired after script execution");
</script>
</body>
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe_javascript_url_xhr.html"
}
```
