# html/semantics/embedded-content/the-iframe-element/move_iframe_in_dom_01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/move_iframe_in_dom_01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>moving modified IFRAME in document (original page about:blank, DOM modification)</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/#iframe-load-event-steps">
<iframe src="about:blank"></iframe>
<div id="target"></div>
<script>
setup({ single_test: true });
onload = function() {
  var ifr = document.getElementsByTagName('iframe')[0];
  ifr.contentDocument.body.appendChild(ifr.contentDocument.createElement('p')).textContent = 'Modified document';
  setTimeout(function() {
    ifr.onload = function() {
      assert_equals(ifr.contentDocument.body.textContent.indexOf('Modified'), -1);
      done();
    };
    document.getElementById('target').appendChild(ifr);
  }, 100);
}
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/move_iframe_in_dom_01.html"
}
```
