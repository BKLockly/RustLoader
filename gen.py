def encode(part):
    return [byte + 4 for byte in part.encode('utf-8')]

def main():
    url = input("请输入 URL: ")

    try:
        protocol, rest = url.split("://")
        host, rest = rest.split(":", 1)
        port, path = rest.split("/", 1)
        path = "/" + path
    except ValueError:
        print("URL 格式不正确，请确保 URL 格式为 http://host:port/path")
        return

    encoded_protocol = encode(protocol)
    encoded_host = encode(host)
    encoded_port = encode(port)
    encoded_path = encode(path)

    print("\n替换为以下部分:")
    print(f"let up = ds(&{encoded_protocol});")
    print(f"let uh = ds(&{encoded_host});")
    print(f"let upo = ds(&{encoded_port});")
    print(f"let upa = ds(&{encoded_path});")


    change_program = input("是否要更换程序路径? (y/n): ").strip().lower()
    if change_program == 'y':
        program_path = input("请输入新的程序路径: ")
        encoded_program = encode(program_path)
        print(f"let p = ds(&[{encoded_program}])")


if __name__ == "__main__":
    main()
