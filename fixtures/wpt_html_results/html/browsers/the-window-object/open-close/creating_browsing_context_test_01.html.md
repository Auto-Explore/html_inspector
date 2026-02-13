# html/browsers/the-window-object/open-close/creating_browsing_context_test_01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/open-close/creating_browsing_context_test_01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>[Browsing Context] : [APIs for creating browsing_contexts by name]</title>
<link rel="author" title="Duhyeong Kim" href="mailto:dduskim@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#apis-for-creating-and-navigating-browsing-contexts-by-name">
<meta name=timeout content=long>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
async_test(function() {
  var currentUrl = 'http://' + window.location.host + '/common/blank.html';
  var win = window.open(currentUrl, '', 'height=1,width=1');
  this.add_cleanup(function() { win.close(); });
  win.onload = this.step_func_done(function () {
    assert_equals(win.location.href, currentUrl, 'should be equal to result url');
  });
}, 'first argument: absolute url');

test(function() {
  var win = window.open('', '', 'height=1,width=1');
  this.add_cleanup(function() { win.close(); });
  assert_equals(win.location.href, 'about:blank', 'win.location.href');
  assert_equals(win.document.charset, 'UTF-8', 'win.document.charset');
}, 'first argument: empty url');

test(function () {
  var win = window.open('', 'testWindow', 'height=1,width=1');
  win.close();
  assert_equals(win.name, 'testWindow', 'should have a browsing context name');
}, 'second argument: passing a non-empty name');

test(function () {
  var win = window.open('', '', 'height=1,width=1');
  this.add_cleanup(function() { win.close(); });
  assert_equals(win.name, '', 'window should not have a name');
  win.name = 'testWindow';
  assert_equals(win.name, 'testWindow', 'window should have a name');
}, 'second argument: setting name after opening');
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
  "source_name": "html/browsers/the-window-object/open-close/creating_browsing_context_test_01.html"
}
```
