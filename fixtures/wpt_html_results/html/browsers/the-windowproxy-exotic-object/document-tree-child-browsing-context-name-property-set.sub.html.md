# html/browsers/the-windowproxy-exotic-object/document-tree-child-browsing-context-name-property-set.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-windowproxy-exotic-object/document-tree-child-browsing-context-name-property-set.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>document-tree child browsing context name property set</title>
<link rel="help" href="https://html.spec.whatwg.org/C/#document-tree-child-browsing-context-name-property-set">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe src="//{{domains[www]}}:{{ports[http][1]}}/common/window-name-setter.html#spices"></iframe>
<iframe name="spices"></iframe>
<iframe name="fruits"></iframe>

<script>
"use strict";
setup({ explicit_done: true });

window.onload = () => {
  test(() => {
    assert_equals(window.spices, undefined);
    assert_not_equals(window.fruits, undefined);
    assert_equals(window.fruits, window[2]);
  }, "Cross origin child window's name shadows the second candidate of a same origin iframe");

  done();
};
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
  "source_name": "html/browsers/the-windowproxy-exotic-object/document-tree-child-browsing-context-name-property-set.sub.html"
}
```
