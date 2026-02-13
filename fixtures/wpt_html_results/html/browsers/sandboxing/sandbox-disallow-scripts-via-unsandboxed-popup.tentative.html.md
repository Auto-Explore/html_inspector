# html/browsers/sandboxing/sandbox-disallow-scripts-via-unsandboxed-popup.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/sandboxing/sandbox-disallow-scripts-via-unsandboxed-popup.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
  <script>
    async_test(t => {
      let i = document.createElement('iframe');
      i.sandbox = "allow-same-origin allow-popups allow-popups-to-escape-sandbox";
      i.srcdoc = `<a target='_blank' rel='opener'
                     href="javascript:window.opener.top.postMessage('FAIL', '*');">Click me!</a>
                  <a target='_blank' rel='opener'
                     href="./resources/post-done-to-opener.html">Click me next!</a>`;

      i.onload = _ => {
        // Since the frame is sandboxed, but allow-same-origin, we can reach into it to grab the
        // anchor element to click. We'll click the `javascript:` URL first, then pop up a new
        // window that posts `DONE`.
        //
        // TODO(mkwst): This feels like a race, but it's one that we consistently win when I'm
        // running the test locally 10,000 times. Good enough!™
        i.contentDocument.body.querySelectorAll('a')[0].click();
        i.contentDocument.body.querySelectorAll('a')[1].click();
      };
      document.body.appendChild(i);

      window.addEventListener('message', t.step_func(e => {
        assert_not_equals(e.data, "FAIL");
        if (e.data == "DONE")
          t.done();
      }));
    }, "Sandboxed => unsandboxed popup");
  </script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/sandboxing/sandbox-disallow-scripts-via-unsandboxed-popup.tentative.html"
}
```
