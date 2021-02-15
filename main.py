from fog05.im.fdu import FDUDescriptor
import sys
from pprint import pprint
import http.client


def press_to_continue():
    input("Press enter to continue")

def read_fdu(filename):

    with open(filename,'r') as file:
        data = file.read()
        if filename.endswith(".json"):
            return FDUDescriptor.deserialize_json(data.encode())
        elif filename.endswith(".yaml"):
            return FDUDescriptor.deserialize_yaml(data.encode())
        else:
            raise ValueError("FDU file should be JSON or YAML")


def send_fdu(fdu):
    conn = http.client.HTTPConnection('127.0.0.1:8080')
    data = fdu.serialize()
    print(f'Serialized: {data}')
    conn.request('POST', '/test/fdu', data)

    resp = conn.getresponse().read()
    return FDUDescriptor.deserialize(resp)


def get_fdu(uuid):
    conn = http.client.HTTPConnection('127.0.0.1:8080')
    conn.request('GET', f'/test/fdu?uuid={uuid}')

    resp = conn.getresponse().read()
    return FDUDescriptor.deserialize(resp)



def main():
    filename = sys.argv[1]
    fdu = read_fdu(filename)

    print(f"FDU Filename: {filename}")
    print(f"FDU: {fdu}")
    press_to_continue()

    print("Sending to rust")
    res = send_fdu(fdu)
    print(f'Response from rust: {res}')
    press_to_continue()


    print('Getting from rust')
    res = get_fdu(res.uuid)
    print(f'Response from rust: {res}')
    press_to_continue()

    print('Bye')


if __name__=='__main__':
    main()