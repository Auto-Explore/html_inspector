# html/webappapis/dynamic-markup-insertion/html-unsafe-methods/setHTMLUnsafe.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/setHTMLUnsafe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/9538">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
for (const containerType of ['Element', 'ShadowRoot']) {
  const createContainer = () => {
    if (containerType == 'ShadowRoot') {
      return document.createElement('div').attachShadow({mode: 'open'});
    } else if (containerType == 'Element') {
      return document.createElement('div');
    }
  };

  test(() => {
    const container = createContainer();
    container.setHTMLUnsafe('<span title=hello>world</span>');

    assert_equals(container.children.length, 1, 'Only one child node should be created.');
    assert_equals(container.firstChild.tagName, 'SPAN', 'The child element should be a span.');
    assert_equals(container.firstChild.getAttribute('title'), 'hello',
      'The title attribute should be set to hello.');
    assert_equals(container.firstChild.childNodes.length, 1,
      'The span should have one child.');
    assert_true(container.firstChild.childNodes[0] instanceof Text,
      'The spans child should be a text node.');
    assert_equals(container.firstChild.textContent, 'world',
      'The spans textContent should be world.');
  }, `${containerType}: setHTMLUnsafe with no shadowdom.`);

  test(() => {
    const container = createContainer();
    container.setHTMLUnsafe(`<div><template shadowrootmode=open><div>hello</div></template></div>`);

    assert_equals(container.children.length, 1, 'One child should be created in the container.');
    const shadowRoot = container.firstChild.shadowRoot;
    assert_true(!!shadowRoot, 'The containers child should have a ShadowRoot.');
    assert_equals(shadowRoot.children.length, 1, 'One child should be created in the ShadowRoot.');
    assert_equals(shadowRoot.firstChild.textContent, 'hello',
      'The ShadowRoots childs textContent should be hello.');
  }, `${containerType}: setHTMLUnsafe with shadowdom.`);
}

test(() => {
  const template = document.createElement('template');
  template.setHTMLUnsafe('<div>hello world</div>');

  assert_equals(template.children.length, 0, 'template should not have any child nodes.');
  assert_equals(template.content.children.length, 1, 'template content should have a child div.');
  assert_equals(template.content.children[0].textContent, 'hello world', 'text content should be set.');
}, 'template.setHTMLUnsafe() should modify template content fragment rather than actual children.');
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
  "source_name": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/setHTMLUnsafe.html"
}
```
