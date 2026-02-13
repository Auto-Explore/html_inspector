# html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/scroll-restoration-basic.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/scroll-restoration-basic.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Verify existence and basic read/write function of history.scrollRestoration</title>

<style>
  body {
    height: 2000px;
    width: 2000px;
  }
</style>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script type="text/javascript">
  'use strict';

  test(function() {
    assert_equals(history.scrollRestoration, 'auto');
  }, 'Default value is "auto"');

  test(function() {
    history.scrollRestoration = 'manual';
    assert_equals(history.scrollRestoration, 'manual', 'should be able to set "manual"');
    history.scrollRestoration = 'auto';
    assert_equals(history.scrollRestoration, 'auto', 'should be able to set "auto"');
  }, 'It is writable');

  test(function() {
    history.scrollRestoration = 'auto';
    for (var v of [3.1415, {}, 'bogus']) {
      history.scrollRestoration = v;
      assert_equals(history.scrollRestoration, 'auto', `setting to invalid value (${v}) should be ignored`);
    }
  }, 'Invalid values are ignored');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 315,
        "byte_start": 284,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/browsing-the-web/history-traversal/persisted-user-state-restoration/scroll-restoration-basic.html"
}
```
