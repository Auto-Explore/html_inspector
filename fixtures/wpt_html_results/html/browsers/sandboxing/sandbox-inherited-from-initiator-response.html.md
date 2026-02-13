# html/browsers/sandboxing/sandbox-inherited-from-initiator-response.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/sandboxing/sandbox-inherited-from-initiator-response.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Inherit sandbox flags from the initiator's response</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
// Check sandbox flags are properly inherited when a document initiate a
// navigation inside another frame that it doesn't own directly.

// This check the sandbox flags defined by the response (e.g. CSP sandbox). See
// also the other test about sandbox flags inherited from the frame.
// => sandbox-inherited-from-initiators-frame.html

// Return a promise, resolving when |element| triggers |event_name| event.
let future = (element, event_name) => {
  return new Promise(resolve => {
    element.addEventListener(event_name, event => resolve(event))
  });
};

promise_test(async test => {
  const iframe_1 = document.createElement("iframe");
  const iframe_2 = document.createElement("iframe");

  iframe_1.id = "iframe_1";
  iframe_2.id = "iframe_2";

  iframe_2.src =
    "./resources/sandbox-inherited-from-initiator-response-helper.html";

  // Insert |iframe_1|. It will load the initial empty document, with no sandbox
  // flags.
  const iframe_1_load_1 = future(iframe_1, "load");
  document.body.appendChild(iframe_1);
  await iframe_1_load_1;

  // Insert |iframe_2|. It will load with sandbox flags. It will make |iframe_1|
  // to navigate toward a data-url, which should inherit the sandbox flags.
  const iframe_1_reply = future(window, "message");
  document.body.appendChild(iframe_2);
  const result = await iframe_1_reply;

  assert_equals("sandboxed", result.data);
})
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
  "source_name": "html/browsers/sandboxing/sandbox-inherited-from-initiator-response.html"
}
```
