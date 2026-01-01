#
import sys
import io
import unittest
import ui


def run_unit_tests():
    """Run unit tests under the `tests` directory and return the textual output."""
    loader = unittest.TestLoader()
    suite = loader.discover(start_dir='tests', pattern='test_*.py')
    buf = io.StringIO()
    runner = unittest.TextTestRunner(stream=buf, verbosity=2)
    result = runner.run(suite)
    return buf.getvalue(), result


if __name__ == '__main__':
    # If invoked with --run-tests, run unit tests and display results above the board
    if '--run-tests' in sys.argv or '--test' in sys.argv:
        output, result = run_unit_tests()
        print('\n=== UNIT TEST OUTPUT ===')
        print(output)
        print('=== SUMMARY ===')
        print(f"Ran: {result.testsRun}, Failures: {len(result.failures)}, Errors: {len(result.errors)}")
        print('\nRunning practice application after tests:\n')
        ui.run_practice_app()
    else:
        ui.run_practice_app()
