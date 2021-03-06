import argparse
import scanner

if __name__ == "__main__":
    parser = argparse.ArgumentParser()

    parser.add_argument("-sP", help="Scan for open ports in given ip")
    parser.add_argument("-sI", help="Scan IP addresses in subnet", action="store_true")

    args = parser.parse_args()

    if args.sP:
        print("Port Scan enabled")
        scanner.PortScanner(args.sP).run()

    if args.sI:
        print("IP Scan enabled")
        scanner.IPScanner().run()