# html/dom/elements/global-attributes/translate-inherit-no-parent-element.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/translate-inherit-no-parent-element.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>The translate attribute inherit state when there's no parent element</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(() => {
  const div = document.createElement("div");
  assert_true(div.translate);
}, 'No parent node');

test(() => {
  const div = document.createElement("div");
  const frag = document.createDocumentFragment();
  frag.append(div);
  assert_true(div.translate);
}, 'DocumentFragment parent node');

for (const translateValue of ['yes', 'no']) {
  test(() => {
    const div = document.createElement("div");
    const myElement = document.createElement("my-element");
    myElement.setAttribute('translate', translateValue);
    myElement.attachShadow({mode: 'open'});
    myElement.shadowRoot.append(div);
    assert_true(div.translate);
  }, `ShadowRoot parent node whose shadow host has translate=${translateValue}`);
}

test(() => {
  assert_true(document.documentElement.translate);
}, 'Document parent node');
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
  "source_name": "html/dom/elements/global-attributes/translate-inherit-no-parent-element.html"
}
```
