# html/webappapis/dynamic-markup-insertion/html-unsafe-methods/setHTMLUnsafe-CEReactions.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/setHTMLUnsafe-CEReactions.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/9957">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
class MyElement extends HTMLElement {
  constructor() {
    super();
    this.numConnectedCallback = 0;
    this.numDisconnectedCallback = 0;
    this.attributeChangedCalls = [];
  }

  connectedCallback() {
    this.numConnectedCallback++;
  }

  disconnectedCallback() {
    this.numDisconnectedCallback++;
  }

  static observedAttributes = ['foo'];
  attributeChangedCallback(name, oldValue, newValue) {
    this.attributeChangedCalls.push({name, oldValue, newValue});
  }
}
customElements.define('my-element', MyElement);

['Element', 'ShadowRoot'].forEach(containerType => {
  test(() => {
    let container = null;
    if (containerType === 'Element') {
      container = document.createElement('div');
      document.body.appendChild(container);
    } else if (containerType === 'ShadowRoot') {
      const host = document.createElement('div');
      document.body.appendChild(host);
      container = host.attachShadow({mode: 'closed'});
    }

    container.setHTMLUnsafe('<my-element>');
    const myElement1 = container.querySelector('my-element');
    assert_equals(myElement1.numConnectedCallback, 1,
      'myElement1.numConnectedCallback after first setHTMLUnsafe.');
    assert_equals(myElement1.numDisconnectedCallback, 0,
      'myElement1.numDisconnectedCallback after first setHTMLUnsafe.');
    assert_equals(JSON.stringify(myElement1.attributeChangedCalls),
      JSON.stringify([]),
      'myElement1.attributeChangedCalls after first setHTMLUnsafe.');

    container.setHTMLUnsafe('<my-element foo=bar>');
    const myElement2 = container.querySelector('my-element');
    assert_equals(myElement1.numConnectedCallback, 1,
      'myElement1.numConnectedCallback after second setHTMLUnsafe.');
    assert_equals(myElement1.numDisconnectedCallback, 1,
      'myElement1.numDisconnectedCallback after second setHTMLUnsafe.');
    assert_array_equals(myElement1.attributeChangedCalls, [],
      'myElement1.attributeChangedCalls after second setHTMLUnsafe.');
    assert_equals(myElement2.numConnectedCallback, 1,
      'myElement2.numConnectedCallback after second setHTMLUnsafe.');
    assert_equals(myElement2.numDisconnectedCallback, 0,
      'myElement2.numDisconnectedCallback after second setHTMLUnsafe.');
    assert_equals(JSON.stringify(myElement2.attributeChangedCalls),
      JSON.stringify([{name: 'foo', oldValue: null, newValue: 'bar'}]),
      'myElement2.attributeChangedCalls after second setHTMLUnsafe.');
  }, `${containerType}.setHTMLUnsafe should trigger custom element reactions.`);
});
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
  "source_name": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/setHTMLUnsafe-CEReactions.html"
}
```
