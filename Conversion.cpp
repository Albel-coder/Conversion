#include <iostream>

using namespace std;

int InputBase = 10;
int OutputBase = 2;
string Number;

inline int GetStringSize(std::string tmp)
{
	short Size = 0;
	for (auto i : tmp)
		Size++;
	return Size;
}

int ConversionToDec(short Size, short InputBaseSystem, short OutputBaseSystem, std::string Str)
{
	unsigned long long Result = 0;
	int Multi = 1;
	int Digit = 0;
	int j = Size - 1; char ch = '0';
	for (int i = 0; i < Size / 2; i++)
	{
		ch = Str[j];
		Str[j] = Str[i];
		Str[i] = ch;
		j--;
	}
	for (int i = 0; i < Size + 1; i++)
	{
		if (i)
		{
			Result += (Digit * Multi);
			Multi *= InputBaseSystem;
		}
		Digit = 0;
		unsigned char tmp = '0';
		for (int c = 0; c < 10; c++)
		{
			if (tmp == Str[i])
				break;
			Digit++;
			tmp++;
		}
		if (tmp == Str[i])
			continue;
		Digit = 10;
		for (unsigned char j = 'A'; j <= 'Z'; j++)
		{
			if (j == Str[i])
				break;
			Digit++;
		}
	}

	return Result;
}

std::string ParseResult(unsigned long long Result, short OutputBaseSystem)
{
	std::string Str = ""; char ch = '0'; int Digit = 0;
	while (Result)
	{
		ch = '0';
		Digit = Result % OutputBaseSystem;
		ch += Digit;
		if (Digit / 10)
		{
			ch += 7;
		}
		Str += ch;
		Result /= OutputBaseSystem;
	}

	int Size = GetStringSize(Str);
	int j = Size - 1; ch = '0';
	for (int i = 0; i < Size / 2; i++)
	{
		ch = Str[j];
		Str[j] = Str[i];
		Str[i] = ch;
		j--;
	}
	return Str;
}

int main()
{
	cout << "Enter system number\n";
	cin >> InputBase;
	cout << "Enter number you want to convert\n";
	cin >> Number;
	cout << "Enter output base\n";
	cin >> OutputBase;
	cout << "Result:\n";
	cout << ParseResult(ConversionToDec(GetStringSize(Number), InputBase, OutputBase, Number), OutputBase) << "\n";

	system("pause");
	return 0;
}