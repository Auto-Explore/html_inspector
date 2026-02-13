# html/webappapis/user-prompts/cannot-show-simple-dialogs/confirm-different-origin-frame.sub.html

Counts:
- errors: 1
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/user-prompts/cannot-show-simple-dialogs/confirm-different-origin-frame.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
  setup({ single_test: true });
  function handleEvent(e) {
    assert_equals(e.data, 'pass');
    done();
  }
  function on_iframe_load() {
    var frameWin = document.getElementById('confirmFrame').contentWindow;
    frameWin.postMessage('Confirm', '*');
  }
  window.addEventListener('message', handleEvent);
</script>

<body>
  <iframe id='confirmFrame'
    src='http://{{hosts[alt][www]}}:{{ports[http][0]}}/html/webappapis/user-prompts/cannot-show-simple-dialogs/support/confirm.html'
    onload='on_iframe_load()'></iframe>
</body>

</html>
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
        "byte_end": 6,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “http://{{hosts[alt][www]}}:{{ports[http][0]}}/html/webappapis/user-prompts/cannot-show-simple-dialogs/support/confirm.html” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 644,
        "byte_start": 455,
        "col": 3,
        "line": 19
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
  "source_name": "html/webappapis/user-prompts/cannot-show-simple-dialogs/confirm-different-origin-frame.sub.html"
}
```
