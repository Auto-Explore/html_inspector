# html/infrastructure/urls/resolving-urls/query-encoding/location.sub.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/urls/resolving-urls/query-encoding/location.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset={{GET[encoding]}}> <!-- ends up as <meta charset> by default which is windows-1252 -->
<meta name=variant content="?encoding=windows-1252">
<meta name=variant content="?encoding=x-cp1251">
<meta name=variant content="?encoding=utf8">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div id=log></div>
<script>
function expected(encoding) {
  return "?" + {
    "UTF-8": "%C3%BF",
    "windows-1251": "%26%23255%3B",
    "windows-1252": "%FF"
  }[encoding];
}

[
  [(win, input) => { win.location = input; }, "location [PutForwards]"],
  [(win, input) => { win.location.assign(input); }, "location.assign()"],
  [(win, input) => { win.location.replace(input); }, "location.replace()"],
  [(win, input) => { win.location.href = input; }, "location.href"]
].forEach(([callback, desc]) => {
  async_test(t => {
    const frame = document.body.appendChild(document.createElement("iframe")),
          actualEncoding = document.characterSet
    callback(frame.contentWindow, "/common/blank.html?\u00FF");
    frame.onload = t.step_func_done(() => {
      assert_equals(frame.contentWindow.location.search, expected(actualEncoding));
    });
  }, desc);
});

async_test(t => {
  const frame = document.body.appendChild(document.createElement("iframe")),
        actualEncoding = document.characterSet;
  frame.src = "/common/blank.html";
  frame.onload = t.step_func(() => {
    frame.contentWindow.location.search = "\u00FF";
    frame.onload = t.step_func_done(() => {
      // location.search always uses UTF-8
      assert_equals(frame.contentWindow.location.search, expected("UTF-8"));
    });
  });
}, "location.search");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “{{GET[encoding]}}” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 48,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
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
  "source_name": "html/infrastructure/urls/resolving-urls/query-encoding/location.sub.html"
}
```
