# html/rendering/non-replaced-elements/hidden-elements.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/hidden-elements.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel=help href="https://html.spec.whatwg.org/#hidden-elements">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div hidden></div>
<embed hidden>
<embed hidden=until-found>
<script>
const kNotHiddenElementLocalNames = [
  "source", "track",
];

const kHiddenElementLocalNames = [
  "area", "base", "basefont", "datalist", "head", "link", "meta", "noembed",
  "noframes", "param", "rp", "script", "style", "template", "title",
];

for (let name of kNotHiddenElementLocalNames) {
  test(function() {
    let element = document.createElement(name);
    document.body.appendChild(element);
    assert_equals(getComputedStyle(element).display, "inline");
  }, `${name} should not be hidden`);
}

for (let name of kHiddenElementLocalNames) {
  test(function() {
    let element = document.createElement(name);
    document.body.appendChild(element);
    assert_equals(getComputedStyle(element).display, "none");
  }, `${name} should be hidden`);
}

test(function() {
  assert_equals(getComputedStyle(document.querySelector("div[hidden]")).display, "none");
}, `div[hidden] element should be hidden`);

test(function() {
  const el = document.querySelector("embed[hidden='']");
  assert_equals(getComputedStyle(el).display, "inline");
  assert_equals(getComputedStyle(el).width, "0px");
  assert_equals(getComputedStyle(el).height, "0px");
}, `embed[hidden=''] element should be inline 0x0`);

test(function() {
  const el = document.querySelector("embed[hidden='until-found']");
  assert_equals(getComputedStyle(el).display, "inline");
  assert_equals(getComputedStyle(el).width, "0px");
  assert_equals(getComputedStyle(el).height, "0px");
  assert_equals(getComputedStyle(el).contentVisibility, "visible");
}, `embed[hidden='until-found'] element should be inline 0x0`);
</script>
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
  "source_name": "html/rendering/non-replaced-elements/hidden-elements.html"
}
```
