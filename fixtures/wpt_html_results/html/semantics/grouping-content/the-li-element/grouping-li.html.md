# html/semantics/grouping-content/the-li-element/grouping-li.html

Counts:
- errors: 0
- warnings: 14
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>li element</title>
    <link rel="author" title="dzenana" href="mailto:dzenana.trenutak@gmail.com">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-li-element">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
</head>
<body>
    <h1>Description</h1>
    <p>This test validates the li element.</p>

    <div id="log"></div>

    <span>
    <menu id="listmenu">
        <li>Command</li>
        <li value="3">Command</li>
    </menu>

    <menu type="toolbar" id="toolbarmenu">
        <li>
            <menu label="File">
                <button type="button">New...</button>
                <button type="button">Open...</button>
            </menu>
        </li>
        <li value="10">
            <menu label="Help">
                <li value = "2"><a href="help.html">Help Me</a></li>
                <li><a href="about.html">About</a></li>
            </menu>
        </li>
    </menu>

    <p>Unordered List</p>
    <ul id="unordered">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ul>
    </span>

    <p>Ordered List</p>
    <ol id="basic">
        <li>list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="start2">
        <li value="2">list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="negative">
        <li value="-10">list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="posFloatDown">
        <li value="4.03">list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="negFloatDown">
        <li value="-4.03">list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="posFloatUp">
        <li value="4.9">list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="negFloatUp">
        <li value="-4.9">list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="exponent">
        <li value="7e2">list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="decimal">
        <li value=".5">list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <p>Ordered List</p>
    <ol id="letter">
        <li value="A">list item</li>
        <li>list item</li>
        <li>list item</li>
    </ol>

    <script>
        "use strict";

        // "The [value] attribute has no default value" so, per https://html.spec.whatwg.org/multipage/#reflect,
        // the default when unspecified is 0
        test(function() {
            var testList = document.querySelectorAll("#unordered li, #basic li");
            for (var i = 0; i < testList.length; i++) {
                assert_equals(testList[i].value, 0, "Default (unspecified) value of value is 0.");
            }
        }, "Default (unspecified) value of value is 0.");

        // "If the value attribute is present, user agents must parse it as an integer, in order to determine the attribute's value.
        //  If the attribute's value cannot be converted to a number, the attribute must be treated as if it was absent."
        // Per https://html.spec.whatwg.org/multipage/#collect-a-sequence-of-characters,
        //    an integer is parsed by collecting as many digits as possible and then aborting at the first
        //    non-digit character after the first digit (otherwise, with no beginning digit, it's just an error)
        // and:  "The value IDL attribute must reflect the value of the value content attribute."

        // start2's first element has value of 2
        test(function() {
            var testLI = document.getElementById("start2").children[0];
            assert_equals(testLI.value, 2, "value of 2 -> value of 2");
        }, ".value property reflects content attribute - and both parse value of '2' correctly.");

        // negative's first element has value of -10
        test(function() {
            var testLI = document.getElementById("negative").children[0];
            assert_equals(testLI.value, -10, "value of -10 -> value of -10");
        }, "IDL and content attribute parse value of '-10' correctly.");

        // posFloatDown's first element has value of 4.03 which should return 4
        test(function() {
            var testLI = document.getElementById("posFloatDown").children[0];
            assert_equals(testLI.value, 4, "value of 4.03 -> 4");
        }, "IDL and content attribute parse value of '4.03' correctly.");

        // negFloatDown's first element has value of -4.03 which should return -4
        test(function() {
            var testLI = document.getElementById("negFloatDown").children[0];
            assert_equals(testLI.value, -4, "value of -4.03 -> -4");
        }, "IDL and content attribute parse value of '-4.03' correctly.");

        // posFloatUp's first element has value of 4.9 which should return 4
        test(function() {
            var testLI = document.getElementById("posFloatUp").children[0];
            assert_equals(testLI.value, 4, "value of 4.9 -> 4");
        }, "IDL and content attribute parse value of '4.9' correctly.");

        // negFloatUp's first element has value of -4.9 which should return -4
        test(function() {
            var testLI = document.getElementById("negFloatUp").children[0];
            assert_equals(testLI.value, -4, "value of -4.9 -> -4");
        }, "IDL and content attribute parse value of '-4.9' correctly.");

        // exponent's first element has value of 7e2 which should return 7
        test(function() {
            var testLI = document.getElementById("exponent").children[0];
            assert_equals(testLI.value, 7, "value of 7e2 -> 7");
        }, "IDL and content attribute parse value of '7e2' correctly.");

        // decimal's first element has value of .5 which should return 0
        test(function() {
            var testLI = document.getElementById("decimal").children[0];
            assert_equals(testLI.value, 0, "value of .5 -> 0 (default)");
        }, "IDL and content attribute parse value of '.5' correctly.");

        // letter's first element has value of A which should return 0
        test(function() {
            var testLI = document.getElementById("letter").children[0];
            assert_equals(testLI.value, 0, "value of A -> 0 (default)");
        }, "IDL and content attribute parse value of 'A' correctly.");

        // SHOULD I TEST MORE NON-ASCII-DIGIT ENTRIES?

    </script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 563,
        "byte_start": 549,
        "col": 9,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “button” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 715,
        "byte_start": 693,
        "col": 17,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “button” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 769,
        "byte_start": 747,
        "col": 17,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 843,
        "byte_start": 828,
        "col": 9,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 908,
        "byte_start": 892,
        "col": 17,
        "line": 32
      }
    },
    {
      "category": "Html",
      "code": "html.p.parent_span",
      "message": "Element “p” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1055,
        "byte_start": 1052,
        "col": 5,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.invalid",
      "message": "Bad value “4.03” for attribute “value” on element “li”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1712,
        "byte_start": 1695,
        "col": 9,
        "line": 69
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.invalid",
      "message": "Bad value “-4.03” for attribute “value” on element “li”.",
      "severity": "Warning",
      "span": {
        "byte_end": 1869,
        "byte_start": 1851,
        "col": 9,
        "line": 76
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.invalid",
      "message": "Bad value “4.9” for attribute “value” on element “li”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2022,
        "byte_start": 2006,
        "col": 9,
        "line": 83
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.invalid",
      "message": "Bad value “-4.9” for attribute “value” on element “li”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2176,
        "byte_start": 2159,
        "col": 9,
        "line": 90
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.invalid",
      "message": "Bad value “7e2” for attribute “value” on element “li”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2327,
        "byte_start": 2311,
        "col": 9,
        "line": 97
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.invalid",
      "message": "Bad value “.5” for attribute “value” on element “li”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2476,
        "byte_start": 2461,
        "col": 9,
        "line": 104
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.invalid",
      "message": "Bad value “A” for attribute “value” on element “li”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2623,
        "byte_start": 2609,
        "col": 9,
        "line": 111
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li.html"
}
```
