def my_func(*args, **kwargs):
    import sys
    print(f'Hello from Python (libpython3.11.a / {sys.version}) in Wasm(Rust).\nargs={args}\n')

    # --- Person demo ---
    import person
    people = []
    for name, age, tags in args:
        p = person.Person(name, age)
        for t in tags:
            p.add_tag(t)
        people.append(p)

    from pprint import pprint as pp
    filter_tag = 'student'
    filtered = person.filter_by_tag(people, filter_tag)
    print('Original people:')
    pp(people)
    print(f'Filtered people by `{filter_tag}`:')
    pp(filtered)

    # --- Decimal demo ---
    import decimal_rs
    print("\nNow testing decimal_rs module...")

    a = decimal_rs.Decimal("10.5")
    b = decimal_rs.Decimal("2")

    print(f"a = {a}, b = {b}")
    print(f"a + b = {a.add(b)}")
    print(f"a - b = {a.sub(b)}")
    print(f"a * b = {a.mul(b)}")
    print(f"a / b = {a.div(b)}")
    print(f"a.to_f64() = {a.to_f64()}")
